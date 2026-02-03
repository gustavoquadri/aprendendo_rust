use tokio::io::AsyncReadExt;
use log::Level;
use tokio::time::sleep;
use tokio::time::Duration;

async fn sleeper(){
    log::info!("Sleeping");
    sleep(Duration::from_secs(1)).await;
    log::info!("Awake!");
}

async fn reader(){
    log::info!("Reading some beeg data");
    // unwrap é usado quando se tem certeza absoluta que existem
    // valores em uma determinada variavel e que voce queira
    // extrair-los rapidamente, assim ele funciona como um
    // atalho para o tratamento de erros (caso o retorno for
    // NONE o programa entra em panic e fecha imediatamente)
    let mut f = tokio::fs::File::open(r"C:\Users\gtv\Desktop\gustavo\projetos\aprendendo_rust\teste_projeto_oficina\src\beeg.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read beeg {} bytes", contents.len());
}

async fn run(){
    // .await usado para nao bloquear a thread, deixando ela
    // fazer outras funções/coisas ao mesmo tempo
    // somente usada em async
    log::info!("-------------------");
    log::info!("Test using .await");
    sleeper().await;
    reader().await;

    // log::info!(r"Next Test! Now using the function called join!\n
    // Used for the same thread (concorrente), I/O assincrono rapido\n
    // operações DEPENDENTES");
    log::info!("-------------------");
    log::info!("Test using tokio:join!");
    tokio::join!(
        sleeper(),
        reader(),
    );

    // log::info!(r"Other test! Now using the function called spawn\n
    // Used for the other threads (paralela), tarefas INDEPENDENTES\n
    // CPU-intensive");
    log::info!("-------------------");
    log::info!("Test using tokio:spawn");
    let sleeper_variavel = tokio::spawn(sleeper());
    let reader_variavel = tokio::spawn(reader());

    let result_sleeper = sleeper_variavel.await.unwrap();
    let result_reader = reader_variavel.await.unwrap();
    

    
}



fn main(){

    simple_logger::init_with_level(Level::Info).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run();

    rt.block_on(future);

}