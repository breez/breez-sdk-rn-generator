{%- for type_ in ci.iter_types() %}
{%- let type_name = type_|type_name %}
{%- match type_ %}
{%- when Type::Record ( name ) %}
    {%- include "Record.swift" %}
{%- when Type::Enum ( name ) %}
    {%- include "Enum.swift" %}
{%- else %}
{%- endmatch -%}    

{%- endfor %}