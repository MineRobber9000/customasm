use crate::*;


#[derive(Debug)]
pub struct DataElement
{
    pub item_ref: util::ItemRef<Self>,
    pub position_within_bank: Option<usize>,
    pub encoding: util::BigInt,
}


pub fn define(
    _report: &mut diagn::Report,
    ast: &mut asm::AstTopLevel,
    _decls: &mut asm::ItemDecls,
    defs: &mut asm::ItemDefs)
    -> Result<(), ()>
{
    for any_node in &mut ast.nodes
    {
        if let asm::AstAny::DirectiveData(ref mut ast_data) = any_node
        {
            for expr in &ast_data.elems
            {
                let item_ref = defs.data_elems.next_item_ref();

                let size = {
                    match ast_data.elem_size
                    {
                        Some(s) => Some(s),
                        None => expr.get_static_size(
                            &expr::StaticSizeInfo::new()),
                    }
                };


                let data_block = DataElement {
                    item_ref,
                    position_within_bank: None,
                    encoding: util::BigInt::new(
                        0,
                        Some(size.unwrap_or(0))),
                };
                
                defs.data_elems.define(item_ref, data_block);
                    
                ast_data.item_refs.push(item_ref);
            }
        }
    }


    Ok(())
}