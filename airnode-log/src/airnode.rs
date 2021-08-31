use hex_literal::hex;
use web3::types::{H160, U256};
use serde::{Deserialize, Serialize};
use crate::logreader::{LogReader, EventParseError};
use phf::phf_map;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AirnodeEvent {
    ClientEndorsementStatusUpdated{
        requester_index: U256,
        client_address: H160,
        endorsement_status: bool,
    },
    ClientFullRequestCreated {
        provider_id: U256,
        request_id: U256,
        no_requests: U256,
        client_address: H160,
        endpoint_id: U256,
        requester_index: U256,
        designated_wallet: H160,
        fulfill_address: H160,
        fulfill_function_id: U256,
        parameters: Vec<U256>,
    },
    ClientRequestFulfilled {
        provider_id: U256,
        request_id: U256,
        status_code: U256,
        data: U256,
    },
    EndpointUpdated{ 
        provider_id: U256,
        endpoint_id: U256,
        authorizers: Vec<H160>,
    },
    ProviderCreated{
        provider_id: U256,
        admin: H160,
        xpub: String,
    },
    RequesterCreated{
        requester_index: U256,
        admin: H160,
    },
    // unknown, but ignored, do not fail on this type
    Unclassified,
    // unknown and fail on that
    Unknown,
}

static KNOWN_EVENTS: phf::Map<&'static str, &'static str> = phf_map! {
    "cee45ff00381d88710040af9bd1c13ee3176f83efd36b33f2330cca74e53000b" => "AirnodeParametersSet(bytes32,address,string,address[])",
    "450d1b4c87d285de4ba1f262cca585d6bb3184ca909e14263ac31a044682d429" => "CheckAuthorizationStatus(bytes32,bytes32,bytes32,uint256,address,address)",
    "c39b3f29cca3bbf148a7d996d34223a627c7df660f04f3d28e915ae11eea446c" => "CheckAuthorizationStatuses(bytes32,bytes32[],bytes32[],uint256[],address[],address[])",
    "d859fb74ec5a83da09d56bb4211667f87fc36cdec8ccdd9ddc7115c99923e75e" => "ClientEndorsementStatusSet(uint256,address,bool)",
    "8acbd28af1fec329994543393007c74ebc717caab62689ba09fbf938f015d3fc" => "ClientEndorsementStatusUpdated(uint256,address,bool)",
    "e8ae99161b1547fd1c6ff3cb9660293fa4cd770fd52f72ff0362d64d8bccc08e" => "ClientFullRequestCreated(bytes32,bytes32,uint256,uint256,address,bytes32,uint256,address,address,bytes4,bytes)",
    "775e78a8e7375d14ad03d31edd0a27b29a055f732bca987abfe8082c16ed7e44" => "ClientFullRequestCreated(bytes32,bytes32,uint256,address,bytes32,uint256,address,address,bytes4,bytes)",
    "8339fddbb81e588a9ed04dec82ee9ae6c7a185f44835adaaa2ace50ce3a14aaf" => "ClientRequestCreated(bytes32,bytes32,uint256,uint256,address,bytes32,uint256,address,address,bytes4,bytes)",
    "aff6f5e5548953a11cbb1cfdd76562512f969b0eba0a2163f2420630d4dda97b" => "ClientRequestCreated(bytes32,bytes32,uint256,address,bytes32,uint256,address,address,bytes4,bytes)",
    "1cfdd5ace64f15111ef8ed9df04364d0e9a9165cccf8386109347e54661ba3ad" => "ClientRequestFailed(bytes32,bytes32)",
    "cde46e28d8d3e348e5f5b4fcc511fe3b1f9b0f549cd8332f0da31802a6f2bf61" => "ClientRequestFulfilled(bytes32,bytes32,uint256,bytes)",
    "1bdbe9e5d42a025a741fc3582eb3cad4ef61ac742d83cc87e545fbd481b926b5" => "ClientRequestFulfilled(bytes32,bytes32,uint256,bytes32)",
    "0ebeb9b9b5c4baf915e7541c7e0919dd1a58eb06ee596035a50d08d20b9219de" => "ClientRequestFulfilledWithBytes(bytes32,bytes32,uint256,bytes)",
    "fcbcd5adb2d26ecd4ad50e6267e977fd479fcd0a6c82bde8eea85290ab3b46e6" => "ClientShortRequestCreated(bytes32,bytes32,uint256,address,bytes32,bytes)",
    "14c9c5c9712ca2ec34c62aaeb946d2762f472ad43612ad1a7fb549e274715e39" => "CreateRequester(address)",
    "57fd46b3414df2b35458ac6ab35236164c21fab7f784d250882d66a197e8cdaf" => "CreateTemplate(bytes32,bytes32,bytes)",
    "ddac55da39d52cd1fe3f9a6832cb6fa4c1fb45d755c47f984cc5a1c48aa774cf" => "DecreasedSelfRank(bytes32,address,uint256)",
    "e5687475d94be4622dec0d6fa4db8686e003947facd485b0f4685954b8e93aa8" => "EndpointUpdated(bytes32,bytes32,address[])",
    "98f45a41d9f64ecabfb00dc5bc4f6aea7e50c2afb9eebee8936be5dbcad03dc2" => "ExtendedWhitelistExpiration(bytes32,address,uint256,address)",
    "3338d71fe7787f614d15e7da994eaa07eb176c0e5193ba05cc0b71cc10017665" => "Fail(bytes32,bytes32,address,bytes4)",
    "8c087e42b178608800a2ea8b3d009bdbbf75e0d23426510c2edd447d4f8b8ebd" => "FailedRequest(address,bytes32)",
    "a3ecafb4214d6c3c8dbf95d5fe8e92051c9ac156e0584a00b2a7e16b2df48c0a" => "Fulfill(bytes32,bytes32,uint256,bytes,address,bytes4)",
    "d1cc11d12363af4b6022e66d14b18ba1779ecd85a5b41891349d530fb6eee066" => "FulfilledRequest(address,bytes32,uint256,bytes)",
    "8ff67212697f6648c4c6355eb59d2e859c8b0de2e42e5c226e43626ec8c38355" => "FulfillWithdrawal(bytes32,bytes32,uint256,address)",
    "67f7af7f4ed3e490e7ed1a528bcee44daf77866210088f1fed035e7a5828515f" => "GetAirnodeParameters(bytes32)",
    "4f69d1b3860fa3a445707db2fc6be4f48e7547d528300db22afa0bd7cb38b10b" => "GetAirnodeParametersAndBlockNumber(bytes32)",
    "b0aa1bf12dcdbed4b0f46468d658a54fa49a871d8f8805a3c7bd9a7f63f10387" => "GetTemplate(bytes32)",
    "06749f6c3a68217036323bcd9899c212a9a9bc94b30e4351f112bb7816e636cb" => "GetTemplates(bytes32[])",
    "eb39930cdcbb560e6422558a2468b93a215af60063622e63cbb165eba14c3203" => "MadeTemplateRequest(address,bytes32,uint256,uint256,address,bytes32,address,address,address,bytes4,bytes)",
    "cf16e219d0e9946ba140541a449711b0d9e7e3600c4fd63f3c5c1e6f7e78f789" => "MakeFullRequest(bytes32,bytes32,uint256,address,address,bytes4,bytes)",
    "bc7e6375ca8aaa60eba7a35af9f5692d6de892654b6d794613be986377467d90" => "MakeRequest(bytes32,uint256,address,address,bytes4,bytes)",
    "40857340078796a2b6bca551f97b62fffe6ae69e2131195d461862224ee871b6" => "MinBalanceUpdated(bytes32,uint256)",
    "36ef18ad81b13124b66c80d27059d75bfadf09474c46aee8bb4ae998a921196d" => "ProviderCreated(bytes32,address,string)",
    "b7de80d002230ae37dd9e25804e78c41517296ad969a962ef5457be94cb8ac6e" => "ProviderUpdated(bytes32,address)",
    "2cb6f3105333165ac08235b122e2651dae9c2e70787572aa65bde31fe838d90d" => "RequestFulfilled(bytes32,uint256,bytes32)",
    "13873a3c5277d69c913bb408d87512468d41afb41113dd46eee917ec4eceb04b" => "RequestFulfilledWithBytes(bytes32,uint256,bytes)",
    "59e98f4c18a6c92efe8c23bcbd74f0d71e271eebf9a95f9edefdbee17c01f270" => "RequesterCreated(uint256,address)",
    "de26d3d8fc98a8dab0df21ef2146d313da1a060d635f3ce9b42adab32fa992aa" => "RequesterUpdated(uint256,address)",
    "19233d41da09723f3b102e7a0e192a478c3cd72aa3269f529824b8cc632043a3" => "RequestWithdrawal(bytes32,uint256,address,address)",
    "ebace4380f1ba3ccf701db78879a937b0ad2a9370e98baaba922228f632383e0" => "SetAirnodeXpub(address,string)",
    "430ace8db43187f56ab9e66a2421b7044fa806a347e0a402a646dc9d0edb0cb1" => "SetApi3Admin(address)",
    "aeae98ebd6f5c18f7e64fdd4102cb25feb3552afa6f81285fb4f942e2e41fc3f" => "SetAirnodeParameters(address,string,address[])",
    "5a0aa1abecb91b072383fcd5858f29ed6135a87dd2f2ab02e8f77def11f19bc2" => "SetAirnodeParametersAndForwardFunds(address,string,address[])",
    "5a570f3df76e0cf7e768a1227f789390627bb125bd73a160596a2559aafdc2da" => "SetClientEndorsementStatus(uint256,address,bool)",
    "19b9e44e700d40233866bbf1eaa4d9465d357e53969945a144ef247f39a0836a" => "SetRank(bytes32,address,uint256,address)",
    "c980cf6eca9a252c38dc7738fb47cd57dfbf37bda6664adfbab67a09df1af859" => "SetRequesterAdmin(uint256,address)",
    "c2e532a12bbcce2bfa2ef9e4bee80180e4e1b1f78618f0d20bc49a648b577c56" => "SetSponsorshipStatus(address,address,bool)",
    "0c4d21ef7140ba56cb38499615ee0b0d3258917f74c778590e597a47ad529ac1" => "SetWhitelistExpiration(bytes32,address,uint256,address)",
    "12abbeaa6fd4b14c4a9208f65d65c03b44c354145ba54d20b11474254438408f" => "SetWhitelistStatusPastExpiration(bytes32,address,bool,address)",
    "a3c44778bf2c4729d112c7eeee01a2a79be70e58e3fe0b2a25d6c3562f73ab83" => "TemplateCreated(bytes32,bytes32,bytes32,bytes)",
    "fa33b8597a1a83305d334562a90f8b4ce657e1b33c081423b6a44792d1cf41a4" => "TemplateCreated(bytes32,bytes32,bytes32,uint256,address,address,bytes4,bytes)",
    "c3dafe3cca75d9d099b1941e05b199870f55b853dd49784a96359ac26f01bf6d" => "TransferredMetaAdminStatus(address)",
    "9e7b58b29aa3b972bb0f457499d0dfd00bf23905b0c3358fb864e7120402aefa" => "WithdrawalFulfilled(bytes32,uint256,bytes32,address,address,uint256)",
    "3d0ebccb4fc9730699221da0180970852f595ed5c78781346149123cbbe9f1d3" => "WithdrawalRequested(bytes32,uint256,bytes32,address,address)",
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

        if t0 == hex!("59e98f4c18a6c92efe8c23bcbd74f0d71e271eebf9a95f9edefdbee17c01f270").into() {
            let mut r = LogReader::new(&log, 1, Some(1)).unwrap();
            return Ok(Self::RequesterCreated{
                requester_index: r.value(),
                admin: r.address(),
            })
        } else if t0 == hex!("8acbd28af1fec329994543393007c74ebc717caab62689ba09fbf938f015d3fc").into() {
            let mut r = LogReader::new(&log, 2, Some(1)).unwrap();
            return Ok(Self::ClientEndorsementStatusUpdated{
                requester_index: r.value(),
                client_address: r.address(),
                endorsement_status: r.bool(),
            })
        } else if t0 == hex!("775e78a8e7375d14ad03d31edd0a27b29a055f732bca987abfe8082c16ed7e44").into() {
            let mut r = LogReader::new(&log, 2, None).unwrap();
            return Ok(Self::ClientFullRequestCreated{
                provider_id: r.value(),
                request_id: r.value(),
                no_requests: r.value(),
                client_address: r.address(),
                endpoint_id: r.value(),
                requester_index: r.value(),
                designated_wallet: r.address(),
                fulfill_address: r.address(),
                fulfill_function_id: r.value(), //.as_u64(),
                parameters: r.values(),
            })
        } else if t0 == hex!("1bdbe9e5d42a025a741fc3582eb3cad4ef61ac742d83cc87e545fbd481b926b5").into() {
            let mut r = LogReader::new(&log, 2, Some(2)).unwrap();
            return Ok(Self::ClientRequestFulfilled{
                provider_id: r.value(),
                request_id: r.value(),
                status_code: r.value(),
                data: r.value(),
            })
        } else if t0 == hex!("e5687475d94be4622dec0d6fa4db8686e003947facd485b0f4685954b8e93aa8").into() {
            let mut r = LogReader::new(&log, 2, None).unwrap();
            return Ok(Self::EndpointUpdated{
                provider_id: r.value(),
                endpoint_id: r.value(),
                authorizers: r.addresses(),
            })
        } else if t0 == hex!("36ef18ad81b13124b66c80d27059d75bfadf09474c46aee8bb4ae998a921196d").into() {
            let mut r = LogReader::new(&log, 1, None).unwrap();
            return Ok(Self::ProviderCreated{
                provider_id: r.value(),
                admin: r.address(),
                xpub: r.text(),
            })
        }

        let topic_str = format!("{:?}", t0).chars().skip(2).collect::<String>();
        match KNOWN_EVENTS.get(&topic_str) {
            Some(title) => {
                tracing::info!("{} topic={:?}", title, t0);
                return Ok(Self::Unclassified)
            },
            None => {
                tracing::warn!("unknown topic {:?}", t0);
            },
        }
        Ok(Self::Unknown)
    }
}