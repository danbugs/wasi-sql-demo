wasmtime::component::bindgen!({
    path: "../wit",
    world: "sql",
    async: true,
});

use host::{add_to_linker, WasiCtx};
use types::Types;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

struct MyStore;
struct MyTypes;

#[async_trait::async_trait]
impl Types for MyTypes {
    async fn prepare_statement(
        &mut self,
        _: std::string::String,
        _: Vec<std::string::String>,
    ) -> std::result::Result<std::result::Result<u32, u32>, anyhow::Error> {
        println!(">>> called prepare_statement");
        Ok(Ok(0))
    }

    async fn drop_statement(&mut self, _: u32) -> std::result::Result<(), anyhow::Error> {
        println!(">>> called drop_statement");
        Ok(())
    }

    async fn open_connection(
        &mut self,
        _: std::string::String,
    ) -> std::result::Result<std::result::Result<u32, u32>, anyhow::Error> {
        println!(">>> called open_connection");
        Ok(Ok(0))
    }

    async fn drop_connection(&mut self, _: u32) -> std::result::Result<(), anyhow::Error> {
        println!(">>> called drop_broker");
        Ok(())
    }

    async fn drop_error(&mut self, _: u32) -> std::result::Result<(), anyhow::Error> {
        println!(">>> called drop_error");
        Ok(())
    }

    async fn trace_error(
        &mut self,
        _: u32,
    ) -> std::result::Result<std::string::String, anyhow::Error> {
        println!(">>> called trace");
        Ok("".to_string())
    }
}

#[async_trait::async_trait]
impl readwrite::Readwrite for MyStore {
    async fn query(
        &mut self,
        _: u32,
        _: u32,
    ) -> std::result::Result<std::result::Result<Vec<types::Row>, u32>, anyhow::Error> {
        println!(">>> called query");
        Ok(Ok(vec![]))
    }
    async fn exec(
        &mut self,
        _: u32,
        _: u32,
    ) -> std::result::Result<std::result::Result<u32, u32>, anyhow::Error> {
        println!(">>> called exec");
        Ok(Ok(0))
    }
}

pub struct Ctx {
    store: MyStore,
    types: MyTypes,
    wasi: WasiCtx,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let store = MyStore;
    let types = MyTypes;

    let wasi = WasiCtxBuilder::new().build();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;

    let mut store = Store::new(&engine, Ctx { store, types, wasi });

    let mut linker = Linker::new(&engine);
    readwrite::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.store)?;
    types::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.types)?;

    add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi)?;

    let component = Component::from_file(&engine, "guest.component.wasm")?;
    let (sql, _) = Sql::instantiate_async(&mut store, &component, &linker).await?;

    println!(">>> calling handle in guest from host");
    let _ = sql.fake_handler.call_handle(&mut store).await?;

    Ok(())
}
