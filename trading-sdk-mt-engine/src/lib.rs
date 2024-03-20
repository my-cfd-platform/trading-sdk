mod dto;
mod flows;
mod caches;
mod test;

pub use dto::*;
pub use flows::*;
pub use caches::*;
pub use test::*;

pub fn sanitize_sl_tp(base_data: &mut MtPositionBaseData){
    if let Some(sl) = base_data.sl_profit{
        if sl > 0.0{
            base_data.sl_profit = Some(-sl);
        }
    }

    if let Some(tp) =  base_data.tp_profit{
        if tp < 0.0{
            base_data.tp_profit = Some(-tp);
        }
    }
}