{%- for type_ in ci.iter_types() %}
{%- let type_name = type_|type_name %}
{%- match type_ %}
{%- when Type::Record ( name ) %}
    {%- include "Record.kt" %}
{%- when Type::Enum ( name ) %}
    {%- include "Enum.kt" %}
{%- else %}
{%- endmatch -%}    

{%- endfor %}