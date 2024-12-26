#[derive(Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Client {
    const BH3_OFFICIAL_UID: u32 = 73565430;
    const WD_OFFICIAL_UID: u32 = 76089447;
    const YS_OFFICIAL_UID: u32 = 75276539;
    const SR_OFFICIAL_UID: u32 = 288909600;
    const ZZZ_OFFICIAL_UID: u32 = 152039148;

    pub fn user_post(&self, user_id: u32) -> crate::api::UserPostAPI {
        crate::api::UserPostAPI {
            client: self.client.clone(),
            user_id,
        }
    }

    pub fn bh3_user_post(&self) -> crate::api::UserPostAPI {
        self.user_post(Self::BH3_OFFICIAL_UID)
    }

    pub fn wd_user_post(&self) -> crate::api::UserPostAPI {
        self.user_post(Self::WD_OFFICIAL_UID)
    }

    pub fn ys_user_post(&self) -> crate::api::UserPostAPI {
        self.user_post(Self::YS_OFFICIAL_UID)
    }

    pub fn sr_user_post(&self) -> crate::api::UserPostAPI {
        self.user_post(Self::SR_OFFICIAL_UID)
    }

    pub fn zzz_user_post(&self) -> crate::api::UserPostAPI {
        self.user_post(Self::ZZZ_OFFICIAL_UID)
    }
}
