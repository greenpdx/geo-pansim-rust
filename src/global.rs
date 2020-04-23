use actix::prelude::*;

use crate::config::Config;
use crate::logging;

pub struct Global {
    //cfg: Rc<Config>,
    //cmd: Addr<Unsync, CommandCenter>,
}

impl Actor for Global {
    type Context = Context<Self>;
}
/*impl Actor for MasterClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}*/

/// Start master process
pub fn start(cfg: Config) -> bool {
    logging::init_logging(&cfg.logging);
    false // init logging
}
