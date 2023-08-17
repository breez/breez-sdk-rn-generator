
{%- macro arg_list(func) %}
    {%- for arg in func.arguments() %}
        {{ arg.name()|var_name }},
    {%- endfor %}
{%- endmacro -%}

{%- macro field_list(rec) %}
    {%- for f in rec.fields() %}
        {{ f.name() }},
    {%- endfor %}
{%- endmacro -%}

{% macro arg_list_decl(func) %}
    {%- for arg in func.arguments() -%}
        {%- match arg.type_() %}      
         {%- when Type::Enum(_) %}
        {{ arg.name()|var_name }}: ReadableMap 
        {%- when Type::Record(_) %}
        {{ arg.name()|var_name }}: ReadableMap
        {%- when Type::Sequence(_) %}
        {{ arg.name()|var_name }}: ReadableArray
        {%- else %}
        {{ arg.name()|var_name }}: {{ arg|type_name -}}
         {%- endmatch %}
        {%- match arg.default_value() %}
        {%- when Some with(literal) %} = {{ literal|render_literal(arg) }}
        {%- else %}
        {%- endmatch %}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}

{% macro return_value(ret_type) %}   
    {%- match ret_type %}
    {%- when Type::Enum(_) %}
    readableMapOf(res)
    {%- when Type::Record(_) %}
    readableMapOf(res)
    {%- when Type::Sequence(_) %}
    readableArrayOf(res)
    {%- else %}
    res
    {%- endmatch %}
{%- endmacro %}