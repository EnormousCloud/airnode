use crate::AirnodeEvent;
use airnode_abi::{Param, ABI};
use ethereum_types::H160;

fn abi_addresses(parameters: &Option<ABI>) -> Vec<H160> {
    let mut out = vec![];
    if let Some(abi) = parameters {
        for p in &abi.params {
            if let Param::Address { value, .. } = p {
                out.push(value.clone());
            }
        }
    }
    out
}

pub fn get_addresses(evt: &AirnodeEvent) -> Vec<H160> {
    match evt {
        AirnodeEvent::ClientEndorsementStatusUpdatedA { client_address, .. } => {
            vec![client_address.clone()]
        }

        AirnodeEvent::ClientFullRequestCreatedA {
            client_address,
            designated_wallet,
            parameters,
            ..
        } => {
            let a = vec![client_address.clone(), designated_wallet.clone()];
            abi_addresses(parameters).iter().cloned().chain(a).collect()
        }
        AirnodeEvent::ClientRequestCreatedA {
            client_address,
            designated_wallet,
            parameters,
            ..
        } => {
            let a = vec![client_address.clone(), designated_wallet.clone()];
            abi_addresses(parameters).iter().cloned().chain(a).collect()
        }
        AirnodeEvent::ClientShortRequestCreatedA {
            client_address,
            parameters,
            ..
        } => {
            let a = vec![client_address.clone()];
            abi_addresses(parameters).iter().cloned().chain(a).collect()
        }
        AirnodeEvent::EndpointUpdatedA { authorizers, .. } => authorizers.clone(),
        AirnodeEvent::ProviderCreatedA { admin, .. } => vec![admin.clone()],
        AirnodeEvent::ProviderUpdatedA { admin, .. } => vec![admin.clone()],
        AirnodeEvent::RequesterCreatedA { admin, .. } => vec![admin.clone()],
        AirnodeEvent::RequesterUpdatedA { admin, .. } => vec![admin.clone()],
        AirnodeEvent::TemplateCreatedA {
            designated_wallet,
            parameters,
            ..
        } => {
            let a = vec![designated_wallet.clone()];
            abi_addresses(parameters).iter().cloned().chain(a).collect()
        }
        AirnodeEvent::WithdrawalFulfilledA {
            designated_wallet,
            destination,
            ..
        } => {
            vec![designated_wallet.clone(), destination.clone()]
        }
        AirnodeEvent::WithdrawalRequestedA {
            designated_wallet,
            destination,
            ..
        } => {
            vec![designated_wallet.clone(), destination.clone()]
        }
        AirnodeEvent::CreatedTemplate {
            airnode,
            parameters,
            ..
        } => {
            let a = vec![airnode.clone()];
            abi_addresses(parameters).iter().cloned().chain(a).collect()
        }
        AirnodeEvent::DecreasedSelfRank { admin, .. } => vec![admin.clone()],
        AirnodeEvent::DecreasedSelfRankAdminned {
            adminned, admin, ..
        } => vec![adminned.clone(), admin.clone()],

        AirnodeEvent::ExtendedWhitelistExpiration {
            airnode,
            user,
            admin,
            ..
        } => vec![airnode.clone(), user.clone(), admin.clone()],
        AirnodeEvent::ExtendedWhitelistExpirationTpl { user, admin, .. } => {
            vec![user.clone(), admin.clone()]
        }
        AirnodeEvent::FailedRequest { airnode, .. } => vec![airnode.clone()],
        AirnodeEvent::FulfilledRequest { airnode, .. } => vec![airnode.clone()],
        AirnodeEvent::FulfilledWithdrawal {
            airnode,
            sponsor,
            sponsor_wallet,
            ..
        } => vec![airnode.clone(), sponsor.clone(), sponsor_wallet.clone()],
        AirnodeEvent::MadeFullRequest {
            airnode,
            requester,
            sponsor,
            sponsor_wallet,
            fulfill_address,
            parameters,
            ..
        } => {
            let a = vec![
                airnode.clone(),
                requester.clone(),
                sponsor.clone(),
                sponsor_wallet.clone(),
                fulfill_address.clone(),
            ];
            abi_addresses(parameters).iter().cloned().chain(a).collect()
        }
        AirnodeEvent::MadeTemplateRequest {
            airnode,
            requester,
            sponsor,
            sponsor_wallet,
            fulfill_address,
            parameters,
            ..
        } => {
            let a = vec![
                airnode.clone(),
                requester.clone(),
                sponsor.clone(),
                sponsor_wallet.clone(),
                fulfill_address.clone(),
            ];
            abi_addresses(parameters).iter().cloned().chain(a).collect()
        }
        AirnodeEvent::RequestedBeaconUpdate {
            sponsor,
            requester,
            sponsor_wallet,
            ..
        } => {
            vec![sponsor.clone(), requester.clone(), sponsor_wallet.clone()]
        }
        AirnodeEvent::RequestedWithdrawal {
            airnode,
            sponsor,
            sponsor_wallet,
            ..
        } => {
            vec![airnode.clone(), sponsor.clone(), sponsor_wallet.clone()]
        }
        AirnodeEvent::SetAirnodeXpub { airnode, .. } => vec![airnode.clone()],
        AirnodeEvent::SetRankAdminned {
            adminned,
            caller_admin,
            target_admin,
            ..
        } => vec![adminned.clone(), caller_admin.clone(), target_admin.clone()],
        AirnodeEvent::SetRank {
            caller_admin,
            target_admin,
            ..
        } => vec![caller_admin.clone(), target_admin.clone()],
        AirnodeEvent::SetSponsorshipStatus {
            sponsor, requester, ..
        } => vec![sponsor.clone(), requester.clone()],

        AirnodeEvent::SetUpdatePermissionStatus {
            sponsor,
            update_requester,
            ..
        } => {
            vec![sponsor.clone(), update_requester.clone()]
        }
        AirnodeEvent::SetWhitelistExpiration {
            airnode,
            user,
            admin,
            ..
        } => {
            vec![airnode.clone(), user.clone(), admin.clone()]
        }
        AirnodeEvent::SetWhitelistExpirationTpl { user, admin, .. } => {
            vec![user.clone(), admin.clone()]
        }
        AirnodeEvent::SetWhitelistStatusPastExpiration {
            airnode,
            user,
            admin,
            ..
        } => vec![airnode.clone(), user.clone(), admin.clone()],
        AirnodeEvent::SetWhitelistStatusPastExpirationTpl { user, admin, .. } => {
            vec![user.clone(), admin.clone()]
        }
        AirnodeEvent::TransferredMetaAdminStatus { meta_admin, .. } => vec![meta_admin.clone()],
        _ => vec![],
    }
}
