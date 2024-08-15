extern create yew;

use yew::prelude::*;

fn main(){
    yew::initialize();
    let app: App<Model> = App:new();
    app.mount_to_body();
    yew::run_loop();
}
