
{% macro arg_list(func) %}
    {%- for arg in func.arguments() -%}
        {{ arg.name()|var_name|unquote -}}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}

{%- macro field_list(rec) %}
    {%- for f in rec.fields() %}
        {{ f.name()|var_name|unquote }},
    {%- endfor %}
{%- endmacro -%}

{% macro arg_list_decl(func) %}
    {%- for arg in func.arguments() -%}
    {{- arg.name()|var_name|unquote }}: {{ arg.type_()|rn_type_name(ci) -}}, {% endfor %}
{%- endmacro %}

{% macro return_value(ret_type) %}   
    {%- match ret_type %}
    {%- when Type::Enum(_) %}readableMapOf(res)
    {%- when Type::Record(_) %}readableMapOf(res)
    {%- when Type::Sequence(_) %}readableArrayOf(res)
    {%- else %}res
    {%- endmatch %}
{%- endmacro %}