{%- for type_ in ci.iter_types() %}
{%- let type_name = type_|type_name %}
{%- match type_ %}
{%- when Type::Object ( name ) %}
{% let obj = ci.get_object_definition(name).unwrap() %}
{%- for meth in obj.methods() -%}
{%- match meth.return_type() -%}
{%- when Some with (return_type) %}
export const {{ meth.name()|fn_name }} = async ({%- call ts::arg_list_decl(meth) -%}): Promise<{{ return_type|type_name }}> => {
    const response = await BreezSDK.{{meth.name()|fn_name}}({%- call ts::arg_list(meth) -%})
    return response
}
{%- when None %}
export const {{ meth.name()|fn_name }} = async ({%- call ts::arg_list_decl(meth) -%}): Promise<void> => {
    await BreezSDK.{{ meth.name()|fn_name }}({%- call ts::arg_list(meth) -%})
}
{%- endmatch %}
{% endfor %}
{%- else -%}
{%- endmatch -%}    
{%- endfor %}

