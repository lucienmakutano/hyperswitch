#[cfg(feature = "payouts")]
use hyperswitch_domain_models::types::{PayoutsData, PayoutsResponseData};
use hyperswitch_domain_models::{
    router_data::{AccessToken, RouterData},
    router_data_v2::{self, flow_common_types::BillingConnectorPaymentsSyncFlowData, RouterDataV2},
    router_flow_types::{
        Accept, AccessTokenAuth, Authorize, BillingConnectorPaymentsSync, Capture, Defend,
        Evidence, PSync, PreProcessing, Session, Upload, Void,
    },
    router_request_types::{
        revenue_recovery::BillingConnectorPaymentsSyncRequest, AcceptDisputeRequestData,
        AccessTokenRequestData, DefendDisputeRequestData, PaymentsAuthorizeData,
        PaymentsCancelData, PaymentsCaptureData, PaymentsPreProcessingData, PaymentsSessionData,
        PaymentsSyncData, RefundsData, SubmitEvidenceRequestData, UploadFileRequestData,
    },
    router_response_types::{
        revenue_recovery::BillingConnectorPaymentsSyncResponse, AcceptDisputeResponse,
        DefendDisputeResponse, PaymentsResponseData, RefundsResponseData, SubmitEvidenceResponse,
        UploadFileResponse,
    },
};

pub(crate) type PaymentsSyncResponseRouterData<R> =
    ResponseRouterData<PSync, R, PaymentsSyncData, PaymentsResponseData>;
pub(crate) type PaymentsResponseRouterData<R> =
    ResponseRouterData<Authorize, R, PaymentsAuthorizeData, PaymentsResponseData>;
pub(crate) type PaymentsCaptureResponseRouterData<R> =
    ResponseRouterData<Capture, R, PaymentsCaptureData, PaymentsResponseData>;
pub(crate) type RefundsResponseRouterData<F, R> =
    ResponseRouterData<F, R, RefundsData, RefundsResponseData>;
pub(crate) type RefreshTokenRouterData =
    RouterData<AccessTokenAuth, AccessTokenRequestData, AccessToken>;

pub(crate) type PaymentsCancelResponseRouterData<R> =
    ResponseRouterData<Void, R, PaymentsCancelData, PaymentsResponseData>;
pub(crate) type PaymentsPreprocessingResponseRouterData<R> =
    ResponseRouterData<PreProcessing, R, PaymentsPreProcessingData, PaymentsResponseData>;
pub(crate) type PaymentsSessionResponseRouterData<R> =
    ResponseRouterData<Session, R, PaymentsSessionData, PaymentsResponseData>;

pub(crate) type AcceptDisputeRouterData =
    RouterData<Accept, AcceptDisputeRequestData, AcceptDisputeResponse>;
pub(crate) type SubmitEvidenceRouterData =
    RouterData<Evidence, SubmitEvidenceRequestData, SubmitEvidenceResponse>;
pub(crate) type UploadFileRouterData =
    RouterData<Upload, UploadFileRequestData, UploadFileResponse>;
pub(crate) type DefendDisputeRouterData =
    RouterData<Defend, DefendDisputeRequestData, DefendDisputeResponse>;

#[cfg(feature = "payouts")]
pub type PayoutsResponseRouterData<F, R> =
    ResponseRouterData<F, R, PayoutsData, PayoutsResponseData>;

pub(crate) type BillingConnectorPaymentsResponseSyncRouterDataV2 = RouterDataV2<
    BillingConnectorPaymentsSync,
    BillingConnectorPaymentsSyncFlowData,
    BillingConnectorPaymentsSyncRequest,
    BillingConnectorPaymentsSyncResponse,
>;

// TODO: Remove `ResponseRouterData` from router crate after all the related type aliases are moved to this crate.
pub struct ResponseRouterData<Flow, R, Request, Response> {
    pub response: R,
    pub data: RouterData<Flow, Request, Response>,
    pub http_code: u16,
}

pub struct  ResponseRouterDataV2<Flow, R, ResourceCommonData, Request, Response> {
    pub response: R,
    pub data: RouterDataV2<Flow, ResourceCommonData, Request, Response>,
    pub http_code: u16,
}
