{%- let rec = ci.get_record_definition(name).unwrap() %}
fun as{{ type_name }}(data: ReadableMap): {{ type_name }}? {
    if (!validateMandatoryFields(data, arrayOf(
        {%- for field in rec.fields() %}
            {%- match field.type_() %} 
            {%- when Type::Optional(_) %}
            {%- else %}
            "{{ field.name()|var_name |unquote }}",
            {%- endmatch %}
        {%- endfor %}
    ))) {
        return null
    }

    {%- for field in rec.fields() %}
    val {{field.name()}} = {{field.type_()|render_from_map(ci, field.name()|var_name|unquote, false)}}    
    {%- endfor %}
    return {{ type_name }}({%- call kt::field_list(rec) -%})    
}

fun readableMapOf({{ type_name|var_name|unquote }}: {{ type_name }}): ReadableMap {
    return readableMapOf(
        {%- for field in rec.fields() %}
        "{{ field.name()|var_name|unquote }}" to {{ field.type_()|render_to_map(ci,type_name|var_name|unquote,field.name()|var_name|unquote,false)}},
        {%- endfor %}       
    )
}

fun as{{ type_name }}List(arr: ReadableArray): List<{{ type_name }}> {
    val list = ArrayList<{{ type_name }}>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(as{{ type_name }}(value)!!)            
            else -> throw IllegalArgumentException("Unsupported type ${value::class.java.name}")
        }
    }
    return list
}