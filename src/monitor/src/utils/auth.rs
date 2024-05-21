// Allowed principals
const ALLOWED_PRINCIPALS: [&str; 8] = [
    // Catalyze development principal
    "syzio-xu6ca-burmx-4afo2-ojpcw-e75j3-m67o5-s5bes-5vvsv-du3t4-wae",
    // Samer II at Candid UI (https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.ic0.app)
    "nvifv-62idm-izjcy-rvy63-7tqjz-mg2d7-jiw6m-soqvp-hdayh-mnqf5-yqe",
    // Samer II at Dashboard (https://ca77u-aiaaa-aaaap-abxiq-cai.icp0.io)
    "bqhkz-ly6mk-7zcms-udvhw-hd7xy-mrlgx-gztdb-i55tr-wpsdz-7g6qc-wae",
    // Remco II at Dashboard (https://ca77u-aiaaa-aaaap-abxiq-cai.icp0.io)
    "q3m5g-guuii-gy5u3-qekcy-rwqxq-ym3ff-zqjue-mjxue-2icoi-qcyk3-pqe",
    // Johan II at Dashboard (https://ca77u-aiaaa-aaaap-abxiq-cai.icp0.io)
    "webky-2c2ee-24zok-g6qc4-twbcw-whjbd-co6e4-ypogf-picwf-ios24-xae",
    // Samer II at localhost:4173
    "xhigk-boo5e-vxlwr-hklcd-ccl57-ltnuh-mq4zy-knsny-fswqd-nad4l-tqe",
    // Semen II at Dashboard (https://ca77u-aiaaa-aaaap-abxiq-cai.icp0.io)
    "cdltk-yqixa-dzz2d-6vdyi-itotf-mj63p-6rfv7-rvm5r-3kxbo-ewseq-dae",
    // Ben II at Dashboard (https://ca77u-aiaaa-aaaap-abxiq-cai.icp0.io)
    "jn5hu-vhbot-bkyej-meqmq-wqkw4-in7jj-vnuuc-odpfw-p5qur-o4fat-yae",
];

pub fn is_registered() -> Result<(), String> {
    let caller = ic_cdk::caller();

    if ALLOWED_PRINCIPALS.contains(&caller.to_string().as_str()) {
        Ok(())
    } else {
        Err(format!(
            "Principal {} is not allowed to call this method",
            caller
        ))
    }
}
