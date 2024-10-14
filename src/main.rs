use salvo::{prelude::*, serve_static::StaticDir};

#[handler]
pub async fn hello_word() -> &'static str {

     "hello word"
}

pub async fn id_params(req: Request) -> i64{

     let id: i64 = req.param::<i64> ("id").unwrap();

     id
}


#[tokio::main] 
async fn main() {
    
     let   router: Router = Router::new()
          .push(Router::new()
          .path("work/<id:/[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}/>")
          .get(id_params))
          .push(Router::new().path("/handle").get(hello_word))     
          .push(Router::new().path("/handle").get(hello_word))
          .push(Router::new().path("/pop/<id>").get(hello_word))
          .push( Router::with_path("<**path>").
          get(StaticDir::new(["public"]).
          defaults("index.html").auto_list(true))
     );

     Server::new(TcpListener::new("127.0.0.1:3000").bind().await).serve(router).await;
}