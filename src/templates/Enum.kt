{%- let e = ci.get_enum_definition(name).unwrap() %}
{%- if e.is_flat() %}

fun as{{ type_name }}(type: String): {{ type_name }} {
    return {{ type_name }}.valueOf(type.uppercase())
}

{% else %}

fun as{{ type_name }}(data: ReadableMap): {{ type_name }}? {
    val type = data.getString("type")

    {% for variant in e.variants() -%}
        if (type == "{{ variant.name()|var_name|unquote }}") {
            {% if variant.has_fields() -%}
            return {{ type_name }}.{{ variant.name() }}( {{ variant.fields()[0].type_()|render_from_map(ci, variant.fields()[0].name()|var_name|unquote, false) }} )                         
            {%- else %}
            return {{ type_name }}.{{ variant.name() }}          
            {%- endif %}       
        }        
    {% endfor %}    

    return null
}

fun readableMapOf({{ type_name|var_name|unquote }}: {{ type_name }}): ReadableMap? {    
    val map = Arguments.createMap()
    when ({{ type_name|var_name|unquote }}) {
    {% for variant in e.variants() -%}        
    is {{ type_name }}.{{ variant.name() }} -> {
        pushToMap(map, "type", "{{ variant.name()|var_name|unquote }}")
        {% for f in variant.fields() -%}
        pushToMap(map, "{{ f.name()|var_name|unquote }}", {{ f.type_()|render_to_map(ci,type_name|var_name|unquote,f.name()|var_name|unquote, false) }})                    
        {%- endfor %}
    }
    {% endfor %}
    }
    return map     
}

{%- endif %}