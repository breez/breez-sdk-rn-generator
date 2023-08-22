{%- let rec = ci.get_record_definition(name).unwrap() %}
static func  as{{ type_name }}(data: [String: Any?]) throws -> {{ type_name }} {   
    {%- for field in rec.fields() %}
    {%- match field.type_() %}         
    {%- when Type::Optional(_) %}
        {% if field.type_()|type_name == field.type_()|map_type_name(ci) -%}
        let {{field.name()|var_name|unquote}} = data["{{field.name()|var_name|unquote}}"] as? {{field.type_()|map_type_name(ci)}}
        {% else -%}
        var {{field.name()|var_name|unquote}}: {{field.type_()|type_name}}
        if let {{field.name()|var_name|unquote|temporary}} = data["{{field.name()|var_name|unquote}}"] as? {{field.type_()|map_type_name(ci)}} {
            {{field.name()|var_name|unquote}} = {{field.type_()|render_from_map(ci, field.name()|var_name|unquote|temporary)}}
        }
        {% endif -%}
    {%- else %}
    {% if field.type_()|type_name == field.type_()|map_type_name(ci) -%}
    guard let {{field.name()|var_name|unquote}} = data["{{field.name()|var_name|unquote}}"] as? {{field.type_()|map_type_name(ci)}} else { throw SdkError.Generic(message: "Missing mandatory field {{field.name()|var_name|unquote}} for type {{ type_name }}") }
    {%- else -%}
    guard let {{field.name()|var_name|unquote|temporary}} = data["{{field.name()|var_name|unquote}}"] as? {{field.type_()|map_type_name(ci)}} else { throw SdkError.Generic(message: "Missing mandatory field {{field.name()|var_name|unquote}} for type {{ type_name }}") }
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
            list.append(try as{{ type_name }}(data: val))
        } else { 
            throw SdkError.Generic(message: "Invalid element type {{ type_name }}")
        }
    }
    return list
}

static func arrayOf({{ type_name|var_name|unquote }}s: [{{ type_name }}]) -> [Any] {
    return {{ type_name|var_name|unquote }}s.map { (v) -> [String: Any?] in return dictionaryOf({{ type_name|var_name|unquote }}: v) }
}