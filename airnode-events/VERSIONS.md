# Protocol Events

This document contains the list that can be recognized by this library

### beta protocol

```
TBD... 
```

### pre-alpha

```
ClientEndorsementStatusUpdated(uint256 requesterIndex, address clientAddress, bool endorsementStatus)
ClientFullRequestCreated(bytes32 providerId, bytes32 requestId, uint256 noRequests, address clientAddress, bytes32 endpointId, uint256 requesterIndex, address designatedWallet, address fulfillAddress,bytes4 ,bytes parameters)
ClientRequestCreated(bytes32 providerId, bytes32 requestId, uint256 noRequests, address clientAddress, bytes32 templateId, uint256 requesterIndex, address designatedWallet, address fulfillAddress, bytes4 functionId, bytes parameters)
ClientRequestFailed(bytes32 providerId, bytes32 requestId)
ClientRequestFulfilled(bytes32 providerId, bytes32 requestId, uint256 statusCode, bytes32 data)
ClientRequestFulfilledWithBytes(bytes32 providerId, bytes32 requestId, uint256 statusCode, bytes data)
ClientShortRequestCreated(bytes32 providerId, bytes32 requestId, uint256 noRequests, address clientAddress, bytes32 templateId, bytes parameters)
EndpointUpdated(bytes32 providerId, bytes32 endpointId, address[] authorizers)
MinBalanceUpdated(bytes32 providerId, uint256 minBalance)
ProviderCreated(bytes32 providerId, address admin, string xpub);
ProviderUpdated(bytes32 providerId, address admin)
RequesterCreated(uint256 requesterIndex, address admin)
RequesterUpdated(uint256 requesterIndex, address admin)
RequestFulfilled(bytes32 requestId, uint256 statusCode, bytes32 data);
RequestFulfilledWithBytes(bytes32 requestId, uint256 statusCode, bytes data);
TemplateCreated(bytes32 indexed templateId, bytes32 providerId, bytes32 endpointId, uint256 requesterIndex, address designatedWallet, address fulfillAddress, bytes4 fulfillFunctionId, bytes parameters)
WithdrawalFulfilled(bytes32 providerId, uint256 requesterIndex, bytes32 withdrawalRequestId, address designatedWallet, address destination, uint256 amount)
WithdrawalRequested(bytes32 providerId, uint256 requesterIndex, bytes32 withdrawalRequestId, address designatedWallet, address destination)
```
