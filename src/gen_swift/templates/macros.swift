{% macro arg_list(func) %}
    {%- for arg in func.arguments() -%}
        {%- match arg.type_() -%}         
        {%- when Type::Record(_) -%}
        {{ arg.name()|var_name|unquote }}: {{ arg.type_()|type_name|var_name|unquote -}}
        {%- else -%}
        {{ arg.name()|var_name|unquote }}: {{ arg.name()|var_name|unquote -}}
        {%- endmatch -%}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}

{% macro arg_list_decl(func) %}
    {%- for arg in func.arguments() -%}
    {{- arg.name()|var_name|unquote }}: {{ arg.type_()|rn_type_name(ci) -}}, {% endfor %}
{%- endmacro %}

{% macro extern_arg_list(func) %}
    {{- func.name()|var_name|unquote -}}:
    {%- for arg in func.arguments() -%}
        {%- if !loop.first -%}
        {{- arg.name()|var_name|unquote }}:
        {%- endif -%}
    {%- endfor %}
    {%- if func.arguments().len() >= 1 -%}resolve:{%- endif -%}reject:
{%- endmacro %}

{%- macro field_list(rec) %}
    {%- for f in rec.fields() %}
        {{ f.name()|var_name|unquote }}: {{ f.name()|var_name|unquote }}{%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro -%}

{% macro return_value(ret_type) %}   
    {%- match ret_type %}
    {%- when Type::Enum(_) %}BreezSDKMapper.dictionaryOf({{ ret_type|type_name|var_name|unquote }}: res)
    {%- when Type::Record(_) %}BreezSDKMapper.dictionaryOf({{ ret_type|type_name|var_name|unquote }}: res)
    {%- when Type::Sequence(inner_type) %}
        {%- match ret_type %}
        {%- when Type::Enum(_) %}BreezSDKMapper.dictionaryOf({{ ret_type|type_name|var_name|unquote }}: res)
        {%- when Type::Record(_) %}BreezSDKMapper.dictionaryOf({{ ret_type|type_name|var_name|unquote }}: res)
        {%- else %}res
        {%- endmatch %}
    {%- else %}res
    {%- endmatch %}
{%- endmacro %}