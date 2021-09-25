pub mod logreader;

use crate::logreader::{EventParseError, LogReader};
use airnode_abi::{DecodingError, ABI};
use hex_literal::hex;
use phf::phf_map;
use serde::{Deserialize, Serialize};
use web3::types::{H160, U256};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AirnodeEvent {
    ClientEndorsementStatusUpdatedA {
        requester_index: U256,
        client_address: H160,
        endorsement_status: bool,
    },
    ClientFullRequestCreatedA {
        provider_id: U256,
        request_id: U256,
        no_requests: u64,
        client_address: H160,
        endpoint_id: U256,
        requester_index: U256,
        designated_wallet: H160,
        fulfill_address: H160,
        fulfill_function_id: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<ABI>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<DecodingError>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Vec<U256>>,
    },
    ClientRequestCreatedA {
        provider_id: U256,
        request_id: U256,
        no_requests: u64,
        client_address: H160,
        template_id: U256,
        requester_index: U256,
        designated_wallet: H160,
        fulfill_address: H160,
        fulfill_function_id: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<ABI>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<DecodingError>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Vec<U256>>,
    },
    ClientRequestFailedA {
        provider_id: U256,
        request_id: U256,
    },
    ClientRequestFulfilledA {
        provider_id: U256,
        request_id: U256,
        status_code: u64,
        data: Vec<U256>,
    },
    ClientRequestFulfilledWithBytesA {
        provider_id: U256,
        request_id: U256,
        status_code: u64,
        data: Vec<U256>,
    },
    ClientShortRequestCreatedA {
        provider_id: U256,
        request_id: U256,
        no_requests: u64,
        client_address: H160,
        template_id: U256,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<ABI>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<DecodingError>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Vec<U256>>,
    },
    EndpointUpdatedA {
        provider_id: U256,
        endpoint_id: U256,
        authorizers: Vec<H160>,
    },
    MinBalanceUpdatedA {
        provider_id: U256,
        min_balance: U256,
    },
    ProviderCreatedA {
        provider_id: U256,
        admin: H160,
        xpub: String,
    },
    ProviderUpdatedA {
        provider_id: U256,
        admin: H160,
    },
    RequesterCreatedA {
        requester_index: U256,
        admin: H160,
    },
    RequesterUpdatedA {
        requester_index: U256,
        admin: H160,
    },
    RequestFulfilledA {
        request_id: U256,
        status_code: u64,
        data: U256,
    },
    RequestFulfilledWithBytesA {
        request_id: U256,
        status_code: u64,
        data: Vec<U256>,
    },
    TemplateCreatedA {
        template_id: U256,
        provider_id: U256,
        endpoint_id: U256,
        requester_index: U256,
        designated_wallet: H160,
        fulfill_address: H160,
        fulfill_function_id: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<ABI>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<DecodingError>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Vec<U256>>,
    },
    WithdrawalFulfilledA {
        provider_id: U256,
        requester_index: U256,
        withdrawal_request_id: U256,
        designated_wallet: H160,
        destination: H160,
        amount: U256,
    },
    WithdrawalRequestedA {
        provider_id: U256,
        requester_index: U256,
        withdrawal_request_id: U256,
        designated_wallet: H160,
        destination: H160,
    },

    CreatedTemplate {
        template_id: U256,
        airnode: H160,
        endpoint_id: U256,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<ABI>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<DecodingError>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Vec<U256>>,
    },
    DecreasedSelfRank {
        admin: H160,
        new_rank: U256,
    },
    DecreasedSelfRankAdminned {
        adminned: H160,
        admin: H160,
        new_rank: U256,
    },
    ErroredBeaconUpdate {
        template_id: U256,
        request_id: U256,
        status_code: u64,
    },
    ExtendedWhitelistExpiration {
        airnode: H160,
        endpoint_id: U256,
        user: H160,
        admin: U256,
        expiration: U256,
    },
    ExtendedWhitelistExpirationTpl {
        template_id: U256,
        user: H160,
        admin: H160,
        expiration: U256,
    },
    FailedRequest {
        airnode: H160,
        request_id: U256,
    },
    FailedRequestMsg {
        airnode: H160,
        request_id: U256,
        error_message: String,
    },
    FulfilledRequest {
        airnode: H160,
        request_id: U256,
        status_code: u64,
        data: Vec<U256>,
    },
    FulfilledWithdrawal {
        airnode: H160,
        sponsor: H160,
        withdrawal_request_id: U256,
        sponsor_wallet: H160,
        amount: U256,
    },
    MadeFullRequest {
        airnode: H160,
        request_id: U256,
        requester_request_count: u64,
        chain_id: u64,
        requester: H160,
        endpoint_id: U256,
        sponsor: H160,
        sponsor_wallet: H160,
        fulfill_address: H160,
        fulfill_function_id: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<ABI>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<DecodingError>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Vec<U256>>,
    },
    MadeTemplateRequest {
        airnode: H160,
        request_id: U256,
        requester_request_count: u64,
        chain_id: u64,
        requester: H160,
        template_id: U256,
        sponsor: H160,
        sponsor_wallet: H160,
        fulfill_address: H160,
        fulfill_function_id: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<ABI>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<DecodingError>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Vec<U256>>,
    },
    RequestedBeaconUpdate {
        template_id: U256,
        sponsor: H160,
        requester: H160,
        request_id: U256,
        sponsor_wallet: H160,
    },
    RequestedWithdrawal {
        airnode: H160,
        sponsor: H160,
        withdrawal_request_id: U256,
        sponsor_wallet: H160,
    },
    SetAirnodeXpub {
        airnode: H160,
        xpub: String,
    },
    SetRankAdminned {
        adminned: H160,
        caller_admin: H160,
        target_admin: H160,
        new_rank: U256,
    },
    SetRank {
        caller_admin: H160,
        target_admin: H160,
        new_rank: U256,
    },
    SetSponsorshipStatus {
        sponsor: H160,
        requester: H160,
        sponsorship_status: bool,
    },
    SetUpdatePermissionStatus {
        sponsor: H160,
        update_requester: H160,
        status: bool,
    },
    SetWhitelistExpiration {
        airnode: H160,
        endpoint_id: U256,
        user: H160,
        admin: H160,
        expiration: U256,
    },
    SetWhitelistExpirationTpl {
        template_id: U256,
        user: H160,
        admin: H160,
        expiration: U256,
    },
    SetWhitelistStatusPastExpiration {
        airnode: H160,
        endpoint_id: U256,
        user: H160,
        admin: H160,
        status: bool,
    },
    SetWhitelistStatusPastExpirationTpl {
        template_id: U256,
        user: H160,
        admin: H160,
        status: bool,
    },
    TransferredMetaAdminStatus {
        meta_admin: H160,
    },
    UpdatedBeacon {
        template_id: U256,
        request_id: U256,
        value: U256,
        timestamp: u64,
    },

    // unknown, but ignored, do not fail on this type
    Unclassified,
    // unknown and fail on that
    Unknown,
}

static KNOWN_EVENTS: phf::Map<&'static str, &'static str> = phf_map! {
    // Pre-Alpha version
    "8acbd28af1fec329994543393007c74ebc717caab62689ba09fbf938f015d3fc" => "ClientEndorsementStatusUpdated(uint256,address,bool)",
    "775e78a8e7375d14ad03d31edd0a27b29a055f732bca987abfe8082c16ed7e44" => "ClientFullRequestCreated(bytes32,bytes32,uint256,address,bytes32,uint256,address,address,bytes4,bytes)",
    "aff6f5e5548953a11cbb1cfdd76562512f969b0eba0a2163f2420630d4dda97b" => "ClientRequestCreated(bytes32,bytes32,uint256,address,bytes32,uint256,address,address,bytes4,bytes)",
    "1cfdd5ace64f15111ef8ed9df04364d0e9a9165cccf8386109347e54661ba3ad" => "ClientRequestFailed(bytes32,bytes32)",
    "1bdbe9e5d42a025a741fc3582eb3cad4ef61ac742d83cc87e545fbd481b926b5" => "ClientRequestFulfilled(bytes32,bytes32,uint256,bytes32)",
    "0ebeb9b9b5c4baf915e7541c7e0919dd1a58eb06ee596035a50d08d20b9219de" => "ClientRequestFulfilledWithBytes(bytes32,bytes32,uint256,bytes)",
    "fcbcd5adb2d26ecd4ad50e6267e977fd479fcd0a6c82bde8eea85290ab3b46e6" => "ClientShortRequestCreated(bytes32,bytes32,uint256,address,bytes32,bytes)",
    "e5687475d94be4622dec0d6fa4db8686e003947facd485b0f4685954b8e93aa8" => "EndpointUpdated(bytes32,bytes32,address[])",
    "40857340078796a2b6bca551f97b62fffe6ae69e2131195d461862224ee871b6" => "MinBalanceUpdated(bytes32,uint256)",
    "36ef18ad81b13124b66c80d27059d75bfadf09474c46aee8bb4ae998a921196d" => "ProviderCreated(bytes32,address,string)",
    "b7de80d002230ae37dd9e25804e78c41517296ad969a962ef5457be94cb8ac6e" => "ProviderUpdated(bytes32,address)",
    "59e98f4c18a6c92efe8c23bcbd74f0d71e271eebf9a95f9edefdbee17c01f270" => "RequesterCreated(uint256,address)",
    "de26d3d8fc98a8dab0df21ef2146d313da1a060d635f3ce9b42adab32fa992aa" => "RequesterUpdated(uint256,address)",
    "2cb6f3105333165ac08235b122e2651dae9c2e70787572aa65bde31fe838d90d" => "RequestFulfilled(bytes32,uint256,bytes32)",
    "13873a3c5277d69c913bb408d87512468d41afb41113dd46eee917ec4eceb04b" => "RequestFulfilledWithBytes(bytes32,uint256,bytes)",
    "fa33b8597a1a83305d334562a90f8b4ce657e1b33c081423b6a44792d1cf41a4" => "TemplateCreated(bytes32,bytes32,bytes32,uint256,address,address,bytes4,bytes)",
    "9e7b58b29aa3b972bb0f457499d0dfd00bf23905b0c3358fb864e7120402aefa" => "WithdrawalFulfilled(bytes32,uint256,bytes32,address,address,uint256)",
    "3d0ebccb4fc9730699221da0180970852f595ed5c78781346149123cbbe9f1d3" => "WithdrawalRequested(bytes32,uint256,bytes32,address,address)",

    // Beta protocol version
    "dfa496c578099ee263f6fbdc842c01815924953f92c186099d640f910c1f92de" => "CreatedTemplate(bytes32,address,bytes32,bytes)",
    "b4a13e8a5b83b6572fd11170aa28965f4b16ce6ed228501322a428b48e34230c" => "DecreasedSelfRank(address,uint256)",
    "907b7436750d9bb04b635c837b151be449230b1975dac4ba31b01343b41eb75c" => "DecreasedSelfRank(address,address,uint256)",
    "df7c6cf6c7d32bf473537bcf24259094d6e7cb863700e071f65a4d8a05b6ce5e" => "ErroredBeaconUpdate(bytes32,bytes32,uint256)",
    "f9b174be67f83278d4516865d1b9ba4576b73e523ea0c2f124ea29152bb1b676" => "ExtendedWhitelistExpiration(address,bytes32,address,address,uint256)",
    "a9e0c89b898eb7a904617915dc5b5510d539c899810042e9248569b54b9cc2ed" => "ExtendedWhitelistExpiration(bytes32,address,address,uint256)",
    "8c087e42b178608800a2ea8b3d009bdbbf75e0d23426510c2edd447d4f8b8ebd" => "FailedRequest(address,bytes32)",
    "c7143b2270cddda57e0087ca5e2a4325657dcab10d10f6b1f9d5ce6b41cb97fc" => "FailedRequest(address,bytes32,string)",
    "d1cc11d12363af4b6022e66d14b18ba1779ecd85a5b41891349d530fb6eee066" => "FulfilledRequest(address,bytes32,uint256,bytes)",
    "adb4840bbd5f924665ae7e0e0c83de5c0fb40a98c9b57dba53a6c978127a622e" => "FulfilledWithdrawal(address,address,bytes32,address,uint256)",
    "3a52c462346de2e9436a3868970892956828a11b9c43da1ed43740b12e1125ae" => "MadeFullRequest(address,bytes32,uint256,uint256,address,bytes32,address,address,address,bytes4,bytes)",
    "eb39930cdcbb560e6422558a2468b93a215af60063622e63cbb165eba14c3203" => "MadeTemplateRequest(address,bytes32,uint256,uint256,address,bytes32,address,address,address,bytes4,bytes)",
    "db6e5ad2f932677d9abcb868239c24d484d5512caf71029b8b7c2309aeee760a" => "RequestedBeaconUpdate(bytes32,address,address,bytes32,address)",
    "d48d52c7c6d0c940f3f8d07591e1800ef3a70daf79929a97ccd80b4494769fc7" => "RequestedWithdrawal(address,address,bytes32,address)",
    "ebace4380f1ba3ccf701db78879a937b0ad2a9370e98baaba922228f632383e0" => "SetAirnodeXpub(address,string)",
    "584a7e3e68feb90397faadcb0af28a855e0268ddedf9fce510b4cf57770b9410" => "SetRank(address,address,address,uint256)",
    "07048cabcdd89c62fecf542621231579eae613db4aeb83794e9c3abf428840ca" => "SetRank(address,address,uint256)",
    "c2e532a12bbcce2bfa2ef9e4bee80180e4e1b1f78618f0d20bc49a648b577c56" => "SetSponsorshipStatus(address,address,bool)",
    "5a3b1968640fbb8b12349ea1a58be5c61eaec6e38c11c38652f1d250207103ab" => "SetUpdatePermissionStatus(address,address,bool)",
    "375ee45428e158031095010484fd6451af89c501c79d75e390da4e91eb480ce1" => "SetWhitelistExpiration(address,bytes32,address,address,uint256)",
    "d19e89b7d547ccf349211588a9a1d29461e2ce984b1b1cdbe7150976528b86f1" => "SetWhitelistExpiration(bytes32,address,address,uint256)",
    "0e8af304f7f920661493a5051df03a3947d58b4f655581e51ab0c014d768d8eb" => "SetWhitelistStatusPastExpiration(address,bytes32,address,address,bool)",
    "527f03e7cb13db04fc83c2332106b6087c66d253bca13289f5d91d8e73796d11" => "SetWhitelistStatusPastExpiration(bytes32,address,address,bool)",
    "c3dafe3cca75d9d099b1941e05b199870f55b853dd49784a96359ac26f01bf6d" => "TransferredMetaAdminStatus(address)",
    "cb9c65e5f99c20826a331174f56e8c536161ecb0b5d598267e79c83567d477f1" => "UpdatedBeacon(bytes32,bytes32,int224,uint32)",

    // Topics from other versions/branches (probably abandoned or deprecated):
    // "cee45ff00381d88710040af9bd1c13ee3176f83efd36b33f2330cca74e53000b" => "AirnodeParametersSet(bytes32,address,string,address[])",
    // "450d1b4c87d285de4ba1f262cca585d6bb3184ca909e14263ac31a044682d429" => "CheckAuthorizationStatus(bytes32,bytes32,bytes32,uint256,address,address)",
    // "c39b3f29cca3bbf148a7d996d34223a627c7df660f04f3d28e915ae11eea446c" => "CheckAuthorizationStatuses(bytes32,bytes32[],bytes32[],uint256[],address[],address[])",
    // "d859fb74ec5a83da09d56bb4211667f87fc36cdec8ccdd9ddc7115c99923e75e" => "ClientEndorsementStatusSet(uint256,address,bool)",
    // "e8ae99161b1547fd1c6ff3cb9660293fa4cd770fd52f72ff0362d64d8bccc08e" => "ClientFullRequestCreated(bytes32,bytes32,uint256,uint256,address,bytes32,uint256,address,address,bytes4,bytes)",
    // "8339fddbb81e588a9ed04dec82ee9ae6c7a185f44835adaaa2ace50ce3a14aaf" => "ClientRequestCreated(bytes32,bytes32,uint256,uint256,address,bytes32,uint256,address,address,bytes4,bytes)",
    // "cde46e28d8d3e348e5f5b4fcc511fe3b1f9b0f549cd8332f0da31802a6f2bf61" => "ClientRequestFulfilled(bytes32,bytes32,uint256,bytes)",
    // "14c9c5c9712ca2ec34c62aaeb946d2762f472ad43612ad1a7fb549e274715e39" => "CreateRequester(address)",
    // "57fd46b3414df2b35458ac6ab35236164c21fab7f784d250882d66a197e8cdaf" => "CreateTemplate(bytes32,bytes32,bytes)",
    // "ddac55da39d52cd1fe3f9a6832cb6fa4c1fb45d755c47f984cc5a1c48aa774cf" => "DecreasedSelfRank(bytes32,address,uint256)",
    // "98f45a41d9f64ecabfb00dc5bc4f6aea7e50c2afb9eebee8936be5dbcad03dc2" => "ExtendedWhitelistExpiration(bytes32,address,uint256,address)",
    // "3338d71fe7787f614d15e7da994eaa07eb176c0e5193ba05cc0b71cc10017665" => "Fail(bytes32,bytes32,address,bytes4)",
    // "a3ecafb4214d6c3c8dbf95d5fe8e92051c9ac156e0584a00b2a7e16b2df48c0a" => "Fulfill(bytes32,bytes32,uint256,bytes,address,bytes4)",
    // "8ff67212697f6648c4c6355eb59d2e859c8b0de2e42e5c226e43626ec8c38355" => "FulfillWithdrawal(bytes32,bytes32,uint256,address)",
    // "cf16e219d0e9946ba140541a449711b0d9e7e3600c4fd63f3c5c1e6f7e78f789" => "MakeFullRequest(bytes32,bytes32,uint256,address,address,bytes4,bytes)",
    // "bc7e6375ca8aaa60eba7a35af9f5692d6de892654b6d794613be986377467d90" => "MakeRequest(bytes32,uint256,address,address,bytes4,bytes)",
    // "19233d41da09723f3b102e7a0e192a478c3cd72aa3269f529824b8cc632043a3" => "RequestWithdrawal(bytes32,uint256,address,address)",
    // "430ace8db43187f56ab9e66a2421b7044fa806a347e0a402a646dc9d0edb0cb1" => "SetApi3Admin(address)",
    // "aeae98ebd6f5c18f7e64fdd4102cb25feb3552afa6f81285fb4f942e2e41fc3f" => "SetAirnodeParameters(address,string,address[])",
    // "5a0aa1abecb91b072383fcd5858f29ed6135a87dd2f2ab02e8f77def11f19bc2" => "SetAirnodeParametersAndForwardFunds(address,string,address[])",
    // "5a570f3df76e0cf7e768a1227f789390627bb125bd73a160596a2559aafdc2da" => "SetClientEndorsementStatus(uint256,address,bool)",
    // "19b9e44e700d40233866bbf1eaa4d9465d357e53969945a144ef247f39a0836a" => "SetRank(bytes32,address,uint256,address)",
    // "c980cf6eca9a252c38dc7738fb47cd57dfbf37bda6664adfbab67a09df1af859" => "SetRequesterAdmin(uint256,address)",
    // "0c4d21ef7140ba56cb38499615ee0b0d3258917f74c778590e597a47ad529ac1" => "SetWhitelistExpiration(bytes32,address,uint256,address)",
    // "12abbeaa6fd4b14c4a9208f65d65c03b44c354145ba54d20b11474254438408f" => "SetWhitelistStatusPastExpiration(bytes32,address,bool,address)",
    // "a3c44778bf2c4729d112c7eeee01a2a79be70e58e3fe0b2a25d6c3562f73ab83" => "TemplateCreated(bytes32,bytes32,bytes32,bytes)",
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirnodeState {
    address: H160,
    network: String,
    events: Vec<AirnodeEvent>,
}

impl AirnodeEvent {
    pub fn from_log(log: &web3::types::Log) -> Result<Self, EventParseError> {
        let t0 = log.topics[0];

        if t0 == hex!("8acbd28af1fec329994543393007c74ebc717caab62689ba09fbf938f015d3fc").into() {
            let mut r = LogReader::new(&log, 2, Some(1)).unwrap();
            return Ok(Self::ClientEndorsementStatusUpdatedA {
                requester_index: r.value(),
                client_address: r.address(),
                endorsement_status: r.bool(),
            });
        } else if t0
            == hex!("775e78a8e7375d14ad03d31edd0a27b29a055f732bca987abfe8082c16ed7e44").into()
        {
            let mut r = LogReader::new(&log, 2, None).unwrap();
            let provider_id = r.value();
            let request_id = r.value();
            let no_requests = r.value().as_u64();
            let client_address = r.address();
            let endpoint_id = r.value();
            let requester_index = r.value();
            let designated_wallet = r.address();
            let fulfill_address = r.address();
            let fulfill_function_id = U256::from(r.value().as_ref()[3] / 0x100000000).as_u64();
            r.skip();
            r.skip();
            let chunks = r.values();
            let (parameters, error, data) = match ABI::decode(&chunks, false) {
                Ok(x) => (Some(x), None, None),
                Err(e) => (None, Some(e), Some(chunks)),
            };
            // chunks.iter().enumerate().for_each(|(i, u)| println!("{:04x?}: {}", i*0x20, serde_json::to_string(u).unwrap()));
            // println!("ClientFullRequestCreated decoded: {:#?}", ABI::decode(&chunks));
            return Ok(Self::ClientFullRequestCreatedA {
                provider_id,
                request_id,
                no_requests,
                client_address,
                endpoint_id,
                requester_index,
                designated_wallet,
                fulfill_address,
                fulfill_function_id, //.as_u64(),
                parameters,
                error,
                data,
            });
        } else if t0
            == hex!("aff6f5e5548953a11cbb1cfdd76562512f969b0eba0a2163f2420630d4dda97b").into()
        {
            let mut r = LogReader::new(&log, 2, None).unwrap();
            let provider_id = r.value();
            let request_id = r.value();
            let no_requests = r.value().as_u64();
            let client_address = r.address();
            let template_id = r.value();
            let requester_index = r.value();
            let designated_wallet = r.address();
            let fulfill_address = r.address();
            let fulfill_function_id = U256::from(r.value().as_ref()[3] / 0x100000000).as_u64();
            r.skip();
            r.skip();
            let chunks = r.values();
            let (parameters, error, data) = match ABI::decode(&chunks, false) {
                Ok(x) => (Some(x), None, None),
                Err(e) => (None, Some(e), Some(chunks)),
            };
            return Ok(Self::ClientRequestCreatedA {
                provider_id,
                request_id,
                no_requests,
                client_address,
                template_id,
                requester_index,
                designated_wallet,
                fulfill_address,
                fulfill_function_id, //.as_u64(),
                parameters,
                error,
                data,
            });
        } else if t0
            == hex!("1cfdd5ace64f15111ef8ed9df04364d0e9a9165cccf8386109347e54661ba3ad").into()
        {
            let mut r = LogReader::new(&log, 2, Some(0)).unwrap();
            return Ok(Self::ClientRequestFailedA {
                provider_id: r.value(),
                request_id: r.value(),
            });
        } else if t0
            == hex!("1bdbe9e5d42a025a741fc3582eb3cad4ef61ac742d83cc87e545fbd481b926b5").into()
        {
            let mut r = LogReader::new(&log, 2, Some(2)).unwrap();
            return Ok(Self::ClientRequestFulfilledA {
                provider_id: r.value(),
                request_id: r.value(),
                status_code: r.value().as_u64(),
                data: r.values(),
            });
        } else if t0
            == hex!("0ebeb9b9b5c4baf915e7541c7e0919dd1a58eb06ee596035a50d08d20b9219de").into()
        {
            let mut r = LogReader::new(&log, 2, None).unwrap();
            return Ok(Self::ClientRequestFulfilledWithBytesA {
                provider_id: r.value(),
                request_id: r.value(),
                status_code: r.value().as_u64(),
                data: r.values(),
            });
        } else if t0
            == hex!("fcbcd5adb2d26ecd4ad50e6267e977fd479fcd0a6c82bde8eea85290ab3b46e6").into()
        {
            let mut r = LogReader::new(&log, 2, None).unwrap();
            let provider_id = r.value();
            let request_id = r.value();
            let no_requests = r.value().as_u64();
            let client_address = r.address();
            let template_id = r.value();
            r.skip();
            r.skip();
            let chunks = r.values();
            let (parameters, error, data) = match ABI::decode(&chunks, false) {
                Ok(x) => (Some(x), None, None),
                Err(e) => (None, Some(e), Some(chunks)),
            };
            return Ok(Self::ClientShortRequestCreatedA {
                provider_id,
                request_id,
                no_requests,
                client_address,
                template_id,
                parameters,
                error,
                data,
            });
        } else if t0
            == hex!("e5687475d94be4622dec0d6fa4db8686e003947facd485b0f4685954b8e93aa8").into()
        {
            let mut r = LogReader::new(&log, 2, None).unwrap();
            return Ok(Self::EndpointUpdatedA {
                provider_id: r.value(),
                endpoint_id: r.value(),
                authorizers: r.addresses(),
            });
        } else if t0
            == hex!("40857340078796a2b6bca551f97b62fffe6ae69e2131195d461862224ee871b6").into()
        {
            let mut r = LogReader::new(&log, 1, Some(1)).unwrap();
            return Ok(Self::MinBalanceUpdatedA {
                provider_id: r.value(),
                min_balance: r.value(),
            });
        } else if t0
            == hex!("36ef18ad81b13124b66c80d27059d75bfadf09474c46aee8bb4ae998a921196d").into()
        {
            let mut r = LogReader::new(&log, 1, None).unwrap();
            return Ok(Self::ProviderCreatedA {
                provider_id: r.value(),
                admin: r.address(),
                xpub: r.text(),
            });
        } else if t0
            == hex!("b7de80d002230ae37dd9e25804e78c41517296ad969a962ef5457be94cb8ac6e").into()
        {
            let mut r = LogReader::new(&log, 1, None).unwrap();
            return Ok(Self::ProviderUpdatedA {
                provider_id: r.value(),
                admin: r.address(),
            });
        } else if t0
            == hex!("59e98f4c18a6c92efe8c23bcbd74f0d71e271eebf9a95f9edefdbee17c01f270").into()
        {
            let mut r = LogReader::new(&log, 1, Some(1)).unwrap();
            return Ok(Self::RequesterCreatedA {
                requester_index: r.value(),
                admin: r.address(),
            });
        } else if t0
            == hex!("de26d3d8fc98a8dab0df21ef2146d313da1a060d635f3ce9b42adab32fa992aa").into()
        {
            let mut r = LogReader::new(&log, 1, Some(1)).unwrap();
            return Ok(Self::RequesterUpdatedA {
                requester_index: r.value(),
                admin: r.address(),
            });
        } else if t0
            == hex!("2cb6f3105333165ac08235b122e2651dae9c2e70787572aa65bde31fe838d90d").into()
        {
            let mut r = LogReader::new(&log, 0, Some(3)).unwrap();
            return Ok(Self::RequestFulfilledA {
                request_id: r.value(),
                status_code: r.value().as_u64(),
                data: r.value(),
            });
        } else if t0
            == hex!("13873a3c5277d69c913bb408d87512468d41afb41113dd46eee917ec4eceb04b").into()
        {
            let mut r = LogReader::new(&log, 0, None).unwrap();
            return Ok(Self::RequestFulfilledWithBytesA {
                request_id: r.value(),
                status_code: r.value().as_u64(),
                data: r.values(),
            });
        } else if t0
            == hex!("fa33b8597a1a83305d334562a90f8b4ce657e1b33c081423b6a44792d1cf41a4").into()
        {
            let mut r = LogReader::new(&log, 1, None).unwrap();
            let template_id = r.value();
            let provider_id = r.value();
            let endpoint_id = r.value();
            let requester_index: U256 = r.value();
            let designated_wallet = r.address();
            let fulfill_address = r.address();
            let fulfill_function_id = U256::from(r.value().as_ref()[3] / 0x100000000).as_u64();
            r.skip();
            r.skip();
            let chunks = r.values();
            let (parameters, error, data) = match ABI::decode(&chunks, false) {
                Ok(x) => (Some(x), None, None),
                Err(e) => (None, Some(e), Some(chunks)),
            };
            return Ok(Self::TemplateCreatedA {
                template_id,
                provider_id,
                endpoint_id,
                requester_index,
                designated_wallet,
                fulfill_address,
                fulfill_function_id,
                parameters,
                error,
                data,
            });
        } else if t0
            == hex!("9e7b58b29aa3b972bb0f457499d0dfd00bf23905b0c3358fb864e7120402aefa").into()
        {
            let mut r = LogReader::new(&log, 3, Some(3)).unwrap();
            return Ok(Self::WithdrawalFulfilledA {
                provider_id: r.value(),
                requester_index: r.value(),
                withdrawal_request_id: r.value(),
                designated_wallet: r.address(),
                destination: r.address(),
                amount: r.value(),
            });
        } else if t0
            == hex!("3d0ebccb4fc9730699221da0180970852f595ed5c78781346149123cbbe9f1d3").into()
        {
            let mut r = LogReader::new(&log, 3, Some(2)).unwrap();
            return Ok(Self::WithdrawalFulfilledA {
                provider_id: r.value(),
                requester_index: r.value(),
                withdrawal_request_id: r.value(),
                designated_wallet: r.address(),
                destination: r.address(),
                amount: r.value(),
            });
        }

        let topic_str = format!("{:?}", t0).chars().skip(2).collect::<String>();
        match KNOWN_EVENTS.get(&topic_str) {
            Some(_title) => {
                // println!("{} topic={:?}", title, t0);
                return Ok(Self::Unclassified);
            }
            None => {
                // println!("unknown topic {:?}", t0);
            }
        }
        Ok(Self::Unknown)
    }
}
