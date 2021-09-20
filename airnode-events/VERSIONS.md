# Protocol Events

This document contains the list of events that can be recognized by this library (collected from multiple branches)

### beta-protocol

```
CreatedTemplate(bytes32 indexed templateId,address airnode,bytes32 endpointId,bytes parameters)
DecreasedSelfRank(address indexed admin, uint256 newRank)
DecreasedSelfRank(address indexed adminned,address indexed admin,uint256 newRank)
ErroredBeaconUpdate(bytes32 indexed templateId,bytes32 requestId,uint256 statusCode)
ExtendedWhitelistExpiration(address indexed airnode,bytes32 endpointId,address indexed user,address indexed admin,uint256 expiration)
ExtendedWhitelistExpiration(bytes32 indexed templateId,address indexed user,address indexed admin,uint256 expiration)
FailedRequest(address indexed airnode, bytes32 indexed requestId)
FulfilledRequest(address indexed airnode,bytes32 indexed requestId,uint256 statusCode,bytes data)
FulfilledWithdrawal(address indexed airnode,address indexed sponsor,bytes32 indexed withdrawalRequestId,address sponsorWallet,uint256 amount);
MadeFullRequest(address indexed airnode,bytes32 indexed requestId,uint256 requesterRequestCount,uint256 chainId,address requester,bytes32 endpointId,address sponsor,address sponsorWallet,address fulfillAddress,bytes4 fulfillFunctionId,bytes parameters)
MadeTemplateRequest(address indexed airnode,bytes32 indexed requestId,uint256 requesterRequestCount,uint256 chainId,address requester,bytes32 templateId,address sponsor,address sponsorWallet,address fulfillAddress,bytes4 fulfillFunctionId,bytes parameters)
RequestedBeaconUpdate(bytes32 indexed templateId,address indexed sponsor,address indexed requester,bytes32 requestId,address sponsorWallet)
RequestedWithdrawal(address indexed airnode,address indexed sponsor,bytes32 indexed withdrawalRequestId,address sponsorWallet)
SetAirnodeXpub(address indexed airnode, string xpub)
SetRank(address indexed adminned,address indexed callerAdmin,address indexed targetAdmin,uint256 newRank)
SetRank(address indexed callerAdmin,address indexed targetAdmin,uint256 newRank)
SetSponsorshipStatus(address indexed sponsor,address indexed requester,bool sponsorshipStatus)
SetUpdatePermissionStatus(address indexed sponsor,address indexed updateRequester,bool status)
SetWhitelistExpiration(address indexed airnode,bytes32 endpointId,address indexed user,address indexed admin,uint256 expiration)
SetWhitelistExpiration(bytes32 indexed templateId,address indexed user,address indexed admin,uint256 expiration)
SetWhitelistStatusPastExpiration(address indexed airnode,bytes32 endpointId,address indexed user,address indexed admin,bool status)
SetWhitelistStatusPastExpiration(bytes32 indexed templateId,address indexed user,address indexed admin,bool status)
TransferredMetaAdminStatus(address indexed metaAdmin)
UpdatedBeacon(bytes32 indexed templateId,bytes32 requestId,int224 value,uint32 timestamp)
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
