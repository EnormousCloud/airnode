use crate::AirnodeEvent;
use web3::types::{H160, U256};

pub fn get_template_id(evt: &AirnodeEvent) -> Option<U256> {
    match evt {
        AirnodeEvent::ClientRequestCreatedA { template_id, .. } => Some(template_id.clone()),
        AirnodeEvent::ClientShortRequestCreatedA { template_id, .. } => Some(template_id.clone()),
        AirnodeEvent::TemplateCreatedA { template_id, .. } => Some(template_id.clone()),
        AirnodeEvent::CreatedTemplate { template_id, .. } => Some(template_id.clone()),
        AirnodeEvent::ErroredBeaconUpdate { template_id, .. } => Some(template_id.clone()),
        AirnodeEvent::ExtendedWhitelistExpirationTpl { template_id, .. } => {
            Some(template_id.clone())
        }
        AirnodeEvent::MadeTemplateRequest { template_id, .. } => Some(template_id.clone()),
        AirnodeEvent::RequestedBeaconUpdate { template_id, .. } => Some(template_id.clone()),
        AirnodeEvent::SetWhitelistExpirationTpl { template_id, .. } => Some(template_id.clone()),
        AirnodeEvent::SetWhitelistStatusPastExpirationTpl { template_id, .. } => {
            Some(template_id.clone())
        }
        AirnodeEvent::UpdatedBeacon { template_id, .. } => Some(template_id.clone()),
        _ => None,
    }
}

pub fn get_endpoint_id(evt: &AirnodeEvent) -> Option<U256> {
    match evt {
        AirnodeEvent::ClientFullRequestCreatedA { endpoint_id, .. } => Some(endpoint_id.clone()),
        AirnodeEvent::EndpointUpdatedA { endpoint_id, .. } => Some(endpoint_id.clone()),
        AirnodeEvent::TemplateCreatedA { endpoint_id, .. } => Some(endpoint_id.clone()),
        AirnodeEvent::CreatedTemplate { endpoint_id, .. } => Some(endpoint_id.clone()),
        AirnodeEvent::ExtendedWhitelistExpiration { endpoint_id, .. } => Some(endpoint_id.clone()),
        AirnodeEvent::MadeFullRequest { endpoint_id, .. } => Some(endpoint_id.clone()),
        AirnodeEvent::SetWhitelistExpiration { endpoint_id, .. } => Some(endpoint_id.clone()),
        AirnodeEvent::SetWhitelistStatusPastExpiration { endpoint_id, .. } => {
            Some(endpoint_id.clone())
        }
        _ => None,
    }
}

pub fn get_fulfill_function_id(evt: &AirnodeEvent) -> Option<u64> {
    match evt {
        AirnodeEvent::ClientFullRequestCreatedA {
            fulfill_function_id,
            ..
        } => Some(fulfill_function_id.clone()),
        AirnodeEvent::ClientRequestCreatedA {
            fulfill_function_id,
            ..
        } => Some(fulfill_function_id.clone()),
        AirnodeEvent::TemplateCreatedA {
            fulfill_function_id,
            ..
        } => Some(fulfill_function_id.clone()),
        AirnodeEvent::MadeFullRequest {
            fulfill_function_id,
            ..
        } => Some(fulfill_function_id.clone()),
        AirnodeEvent::MadeTemplateRequest {
            fulfill_function_id,
            ..
        } => Some(fulfill_function_id.clone()),
        _ => None,
    }
}

pub fn get_provider_id(evt: &AirnodeEvent) -> Option<U256> {
    match evt {
        AirnodeEvent::ClientFullRequestCreatedA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::ClientRequestCreatedA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::ClientRequestFailedA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::ClientRequestFulfilledA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::ClientRequestFulfilledWithBytesA { provider_id, .. } => {
            Some(provider_id.clone())
        }
        AirnodeEvent::ClientShortRequestCreatedA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::EndpointUpdatedA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::MinBalanceUpdatedA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::ProviderCreatedA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::ProviderUpdatedA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::TemplateCreatedA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::WithdrawalFulfilledA { provider_id, .. } => Some(provider_id.clone()),
        AirnodeEvent::WithdrawalRequestedA { provider_id, .. } => Some(provider_id.clone()),
        _ => None,
    }
}

pub fn get_requester_index(evt: &AirnodeEvent) -> Option<U256> {
    match evt {
        AirnodeEvent::ClientEndorsementStatusUpdatedA {
            requester_index, ..
        } => Some(requester_index.clone()),
        AirnodeEvent::ClientFullRequestCreatedA {
            requester_index, ..
        } => Some(requester_index.clone()),
        AirnodeEvent::ClientRequestCreatedA {
            requester_index, ..
        } => Some(requester_index.clone()),
        AirnodeEvent::RequesterCreatedA {
            requester_index, ..
        } => Some(requester_index.clone()),
        AirnodeEvent::RequesterUpdatedA {
            requester_index, ..
        } => Some(requester_index.clone()),
        AirnodeEvent::TemplateCreatedA {
            requester_index, ..
        } => Some(requester_index.clone()),
        AirnodeEvent::WithdrawalFulfilledA {
            requester_index, ..
        } => Some(requester_index.clone()),
        AirnodeEvent::WithdrawalRequestedA {
            requester_index, ..
        } => Some(requester_index.clone()),
        _ => None,
    }
}

pub fn get_request_id(evt: &AirnodeEvent) -> Option<U256> {
    match evt {
        AirnodeEvent::ClientFullRequestCreatedA { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::ClientRequestCreatedA { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::ClientRequestFailedA { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::ClientRequestFulfilledA { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::ClientRequestFulfilledWithBytesA { request_id, .. } => {
            Some(request_id.clone())
        }
        AirnodeEvent::ClientShortRequestCreatedA { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::RequestFulfilledA { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::RequestFulfilledWithBytesA { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::WithdrawalFulfilledA {
            withdrawal_request_id,
            ..
        } => Some(withdrawal_request_id.clone()),
        AirnodeEvent::WithdrawalRequestedA {
            withdrawal_request_id,
            ..
        } => Some(withdrawal_request_id.clone()),
        AirnodeEvent::ErroredBeaconUpdate { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::FailedRequest { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::FulfilledRequest { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::FulfilledRequestWithStatus { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::FulfilledWithdrawal {
            withdrawal_request_id,
            ..
        } => Some(withdrawal_request_id.clone()),

        AirnodeEvent::MadeFullRequest { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::MadeTemplateRequest { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::RequestedBeaconUpdate { request_id, .. } => Some(request_id.clone()),
        AirnodeEvent::RequestedWithdrawal {
            withdrawal_request_id,
            ..
        } => Some(withdrawal_request_id.clone()),
        AirnodeEvent::UpdatedBeacon { request_id, .. } => Some(request_id.clone()),
        _ => None,
    }
}

pub fn get_airnode(evt: &AirnodeEvent) -> Option<H160> {
    match evt {
        AirnodeEvent::CreatedTemplate { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::ExtendedWhitelistExpiration { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::FailedRequest { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::FulfilledRequest { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::FulfilledRequestWithStatus { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::FulfilledWithdrawal { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::MadeFullRequest { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::MadeTemplateRequest { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::RequestedWithdrawal { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::SetAirnodeXpub { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::SetWhitelistExpiration { airnode, .. } => Some(airnode.clone()),
        AirnodeEvent::SetWhitelistStatusPastExpiration { airnode, .. } => Some(airnode.clone()),
        _ => None,
    }
}
