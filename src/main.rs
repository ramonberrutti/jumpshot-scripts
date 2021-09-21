pub mod script_service {
    tonic::include_proto!("script.v1");
}

use script_service::script_service_server::{ScriptService, ScriptServiceServer};
use script_service::{RunRequest, RunResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct ScriptV1 {
    engine: wasmtime::Engine,
}

#[tonic::async_trait]
impl ScriptService for ScriptV1 {
    async fn run(&self, request: Request<RunRequest>) -> Result<Response<RunResponse>, Status> {
        let req = request.get_ref();

        let module = wasmtime::Module::from_binary(&self.engine, &req.script)
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        let mut store = wasmtime::Store::new(&self.engine, ());

        let instance = wasmtime::Instance::new(&mut store, &module, &[])
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        let answer = instance.get_func(&mut store, "answer")
            .ok_or(Status::new(tonic::Code::InvalidArgument, "function not found"))?;

        let answer = answer.typed::<(), i32, _>(&store)
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        let result = answer.call(&mut store, ())
        .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        Ok(Response::new(RunResponse{
            output: format!("answer is {}", result)
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let scriptv1 = ScriptV1::default();

    Server::builder()
        .add_service(ScriptServiceServer::new(scriptv1))
        .serve(addr)
        .await?;

    Ok(())
}
