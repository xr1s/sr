use std::{
    async_iter::AsyncIterator,
    future::Future,
    pin::{pin, Pin},
    task::{ready, Context, Poll},
};

const URL: &str = "https://bbs-api.miyoushe.com/post/wapi/userPost";

pub struct UserPostAPI {
    pub(crate) client: reqwest::Client,
    pub(crate) user_id: u32,
}

pub struct UserPostIter {
    client: reqwest::Client,
    user_id: u32,

    is_last: bool,
    posts: Vec<crate::model::user_post::UserPost>,
    future: UserPostFuture,
}

#[derive(serde::Serialize)]
struct UserPostQuery {
    size: u8,
    uid: u32,
    #[serde(skip_serializing_if = "num::Zero::is_zero")]
    offset: u32,
}

struct UserPostFuture {
    req: Option<Pin<Box<dyn Future<Output = reqwest::Result<reqwest::Response>>>>>,
    body: Option<Pin<Box<dyn Future<Output = reqwest::Result<bytes::Bytes>>>>>,
}

impl UserPostFuture {
    fn new(client: &reqwest::Client, query: &UserPostQuery) -> Self {
        Self {
            req: Some(Box::pin(client.get(URL).query(&query).send())),
            body: None,
        }
    }
}

impl Future for UserPostFuture {
    type Output = reqwest::Result<bytes::Bytes>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(req) = self.req.as_mut() {
            let res = ready!(req.as_mut().poll(cx))?.error_for_status()?;
            self.body = Some(Box::pin(res.bytes()));
            self.req = None;
        }
        if let Some(body) = self.body.as_mut() {
            return body.as_mut().poll(cx);
        }
        unreachable!()
    }
}

impl UserPostAPI {
    pub fn async_iter(&self) -> UserPostIter {
        let query = UserPostQuery {
            size: 50,
            uid: self.user_id,
            offset: 0,
        };
        UserPostIter {
            client: self.client.clone(),
            user_id: self.user_id,
            is_last: false,

            posts: Vec::new(),
            future: UserPostFuture::new(&self.client, &query),
        }
    }
}

impl AsyncIterator for UserPostIter {
    type Item = crate::api::Result<crate::model::user_post::UserPost>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(post) = self.posts.pop() {
            return Poll::Ready(Some(Ok(post)));
        }
        if self.is_last {
            return Poll::Ready(None);
        }
        let bytes = ready!(pin!(&mut self.future).poll(cx))?;
        let res: crate::model::Response<crate::model::user_post::UserPostList> =
            serde_json::from_slice(&bytes)?;
        if res.retcode != 0 {
            return Poll::Ready(Some(Err(crate::api::Error::Business {
                retcode: res.retcode,
                message: res.message,
            })));
        }
        let query = UserPostQuery {
            size: 50,
            uid: self.user_id,
            offset: res.data.next_offset,
        };
        self.future = UserPostFuture::new(&self.client, &query);
        self.is_last = res.data.is_last;
        self.posts = res.data.list;
        self.posts.reverse();
        std::task::Poll::Ready(self.posts.pop().map(Ok))
    }
}
