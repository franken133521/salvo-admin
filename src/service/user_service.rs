use crate::entity::sys_user_entity::SysUser;
use crate::entity::sys_captcha_entity::SysCaptcha;
use crate::GLOBAL_DB;
use rbatis::rbdc::Error;

pub async fn get_user_by_up(username:String,password:String)->Result<Option<SysUser>,Error>{
  // match SysUserEntity::select_user_by_up(&mut GLOBAL_DB.clone(), username, password).await{
  //   Ok(list)=>{
  //     Ok(list.get(0).cloned())
  //   },
  //   Err(err)=>Err(err)
  // }
  let list = SysUser::select_user_by_up(&mut GLOBAL_DB.clone(), username, password).await?;
  let one = list.get(0).cloned();
  Ok(one)
}

#[allow(dead_code)]
pub async fn get_captcha_by_code(code:String)->Result<Option<SysCaptcha>,Error>{
  let list = SysCaptcha::select_captcha_by_code(&mut GLOBAL_DB.clone(), code).await?;
  let one = list.get(0).cloned();
  Ok(one)
}