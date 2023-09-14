{%- let rec = ci.get_record_definition(name).unwrap() %}
static func  as{{ type_name }}(data: [String: Any?]) throws -> {{ type_name }} {   
    {%- for field in rec.fields() %}
    {%- match field.type_() %}         
    {%- when Type::Optional(_) %}
        {% if field.type_()|inline_optional_field(ci) -%}
        let {{field.name()|var_name|unquote}} = data["{{field.name()|var_name|unquote}}"] as? {{field.type_()|rn_type_name(ci, true)}}
        {%- else -%}
        var {{field.name()|var_name|unquote}}: {{field.type_()|type_name}}
        if let {{field.name()|var_name|unquote|temporary}} = data["{{field.name()|var_name|unquote}}"] as? {{field.type_()|rn_type_name(ci, true)}} {
            {{field.name()|var_name|unquote}} = {{field.type_()|render_from_map(ci, field.name()|var_name|unquote|temporary)}}
        }
        {% endif -%}
    {%- else %}
    {% if field.type_()|inline_optional_field(ci) -%}
    guard let {{field.name()|var_name|unquote}} = data["{{field.name()|var_name|unquote}}"] as? {{field.type_()|rn_type_name(ci, true)}} else { throw SdkError.Generic(message: "Missing mandatory field {{field.name()|var_name|unquote}} for type {{ type_name }}") }
    {%- else -%}
    guard let {{field.name()|var_name|unquote|temporary}} = data["{{field.name()|var_name|unquote}}"] as? {{field.type_()|rn_type_name(ci, true)}} else { throw SdkError.Generic(message: "Missing mandatory field {{field.name()|var_name|unquote}} for type {{ type_name }}") }
    let {{field.name()|var_name|unquote}} = {{field.type_()|render_from_map(ci, field.name()|var_name|unquote|temporary)}}
    {% endif -%}        
    {% endmatch %}
    {%- endfor %}
    
    return {{ type_name }}({%- call swift::field_list(rec) -%})    
}

static func  dictionaryOf({{ type_name|var_name|unquote }}: {{ type_name }}) -> [String: Any?] {
    return [
        {%- for field in rec.fields() %}
            "{{ field.name()|var_name|unquote }}": {{ field.type_()|render_to_map(ci,type_name|var_name|unquote,field.name()|var_name|unquote,false)}},
        {%- endfor %}       
    ]
}

static func  as{{ type_name }}List(arr: [Any]) throws -> [{{ type_name }}] {
    var list = [{{ type_name }}]()
    for value in arr {
        if let val = value as? [String: Any?] {
            var {{ type_name|var_name|unquote }} = try as{{ type_name }}(data: val)
            list.append({{ type_name|var_name|unquote }})
        } else { 
            throw SdkError.Generic(message: "Invalid element type {{ type_name }}")
        }
    }
    return list
}

static func arrayOf({{ type_name|var_name|unquote|list_arg }}: [{{ type_name }}]) -> [Any] {
    return {{ type_name|var_name|unquote|list_arg }}.map { (v) -> [String: Any?] in return dictionaryOf({{ type_name|var_name|unquote }}: v) }
}