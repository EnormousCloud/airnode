use crate::AirnodeEvent;
use web3::types::U256;

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
