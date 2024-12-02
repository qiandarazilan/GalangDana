use ic_cdk::api::trap;
use ic_cdk::export::candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Campaign {
    id: u64,
    owner: Principal,
    title: String,
    description: String,
    target_amount: u64,
    collected_amount: u64,
    is_verified: bool,
}

struct GalangDana {
    campaigns: HashMap<u64, Campaign>,
    next_campaign_id: u64,
}

impl GalangDana {
    fn new() -> Self {
        Self {
            campaigns: HashMap::new(),
            next_campaign_id: 1,
        }
    }

    fn create_campaign(
        &mut self,
        owner: Principal,
        title: String,
        description: String,
        target_amount: u64,
    ) -> u64 {
        let campaign = Campaign {
            id: self.next_campaign_id,
            owner,
            title,
            description,
            target_amount,
            collected_amount: 0,
            is_verified: false,
        };
        self.campaigns.insert(self.next_campaign_id, campaign);
        self.next_campaign_id += 1;
        self.next_campaign_id - 1
    }

    fn list_campaigns(&self) -> Vec<Campaign> {
        self.campaigns.values().cloned().collect()
    }

    fn donate(&mut self, campaign_id: u64, amount: u64) -> Result<(), String> {
        if let Some(campaign) = self.campaigns.get_mut(&campaign_id) {
            if campaign.is_verified {
                campaign.collected_amount += amount;
                Ok(())
            } else {
                Err("Kampanye belum diverifikasi".into())
            }
        } else {
            Err("Kampanye tidak ditemukan".into())
        }
    }

    fn verify_campaign(&mut self, campaign_id: u64) -> Result<(), String> {
        if let Some(campaign) = self.campaigns.get_mut(&campaign_id) {
            campaign.is_verified = true;
            Ok(())
        } else {
            Err("Kampanye tidak ditemukan".into())
        }
    }
}

static mut GALANG_DANA: Option<GalangDana> = None;

#[ic_cdk::init]
fn init() {
    unsafe {
        GALANG_DANA = Some(GalangDana::new());
    }
}

#[ic_cdk::update]
fn create_campaign(title: String, description: String, target_amount: u64) -> u64 {
    let caller = ic_cdk::caller();
    let app = unsafe { GALANG_DANA.as_mut().unwrap() };
    app.create_campaign(caller, title, description, target_amount)
}

#[ic_cdk::query]
fn list_campaigns() -> Vec<Campaign> {
    let app = unsafe { GALANG_DANA.as_ref().unwrap() };
    app.list_campaigns()
}

#[ic_cdk::update]
fn donate(campaign_id: u64, amount: u64) -> Result<(), String> {
    let app = unsafe { GALANG_DANA.as_mut().unwrap() };
    app.donate(campaign_id, amount)
}

#[ic_cdk::update]
fn verify_campaign(campaign_id: u64) -> Result<(), String> {
    let app = unsafe { GALANG_DANA.as_mut().unwrap() };
    app.verify_campaign(campaign_id)
}
