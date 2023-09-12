#![feature(impl_trait_in_assoc_type)]
use anyhow::Error;
use std::collections::HashMap;
use std::cell::RefCell;
pub struct S{
    pair:RefCell<HashMap<String,String>>,
}
unsafe impl Send for S {}
unsafe impl Sync for S {}
impl S{
	pub fn new()->S{
		S{
			pair:RefCell::new(HashMap::<String,String>::new()),
		}
	}
}
#[volo::async_trait]
impl volo_gen::miniredis::ItemService for S {

    async fn get_item(&self,_req: volo_gen::miniredis::GetItemRequest) 
    -> core::result::Result<volo_gen::miniredis::GetItemResponse, volo_thrift::AnyhowError>
    {
        let mut resp = volo_gen::miniredis::GetItemResponse {tyep: 0, key: _req.key.clone(), value: " ".into(), success: false};
        let key:String=_req.key.into();
		let mut pair=self.pair.borrow_mut();
        let num:i32=_req.tyep.into();
        match num{
            0=>{
                
                if pair.contains_key(&key){
                    resp.tyep=0;
                    resp.key=key.clone().into();
                    resp.value=pair[&key].clone().into();
                    resp.success=true;
                }
                else{
                    resp.tyep=0;
                    resp.key=key.into();
                    resp.success=false;
                }
            }
            1=>{//set,存在则重新赋值，不存在则新建
                let value:String=_req.value.into();
                if pair.contains_key(&key){//重新赋值
                    pair.insert(key.clone(),value.clone());
                    resp.tyep=1;
                    resp.key=key.clone().into();
                    resp.value=value.clone().into();
                    resp.success=false;
                }
                else{//新建
                    pair.insert(key.clone(),value.clone());
                    resp.tyep=1;
                    resp.key=key.clone().into();
                    resp.value=value.clone().into();
                    resp.success=true;
                    
                }
            }
            2=>{
                if pair.contains_key(&key){
                    pair.remove(&key);
                    resp.tyep=2;
                    resp.key=key.clone().into();
                    resp.success=true;
                }
                else{
                    resp.tyep=2;
                    resp.key=key.clone().into();
                    resp.success=false;
                    
                }
            }
            3=>{
                resp.tyep=3;
                resp.success=true;
            }
			_=>{}
        }
        Ok(resp)
    
    
    }
    
}
//---------------------------------------
#[derive(Clone)]
pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}



#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}
//--------------------------------------
pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FilterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FilterService(inner)
    }
}



#[derive(Clone)]
pub struct FilterService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug+ From<Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let info = format!("{:?}", req);
		if info.contains("test") {
			Err(S::Error::from(Error::msg("filter working")))
		} else {
			self.0.call(cx, req).await
		}
    }
}