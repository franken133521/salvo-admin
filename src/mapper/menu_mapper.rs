use rbatis::{html_sql, executor::Executor};
use crate::entity::sys_menu_entity::SysMenu;

#[html_sql("src/mapper/xml/menu_xml.html")]
pub async fn select_menus_by_role_id(rb: &mut dyn Executor,is_admin:bool,id:String)->rbatis::Result<Vec<SysMenu>>{
  impled!()
}

#[html_sql("src/mapper/xml/menu_xml.html")]
pub async fn select_menus_by_user_id(rb: &mut dyn Executor,is_admin:bool,id:i32)->rbatis::Result<Vec<SysMenu>>{
  impled!()
}