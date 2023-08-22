export const {{func.name()|fn_name}} = async ({%- call ts::arg_list_decl(func) -%}): Promise<string> => {
    const response = await BreezSDK.{{func.name()|fn_name}}({%- call ts::arg_list(func) -%})
    return response
}
