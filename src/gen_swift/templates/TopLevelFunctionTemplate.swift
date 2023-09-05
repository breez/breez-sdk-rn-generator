    
    @objc({%- call swift::extern_arg_list(func) -%})
    func {{ func.name()|fn_name|unquote }}(_ {% call swift::arg_list_decl(func) -%}resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
{%- for arg in func.arguments() -%}
    {%- match arg.type_() %}
    {%- when Type::Enum(_) %}
            let {{arg.name()|var_name|unquote|temporary}} = try BreezSDKMapper.as{{arg.type_()|type_name}}(type: {{ arg.name()|var_name|unquote }})
    {%- when Type::Optional(_) %}
            let {{arg.name()|var_name|unquote|temporary}} = {{ arg.type_()|rn_convert_type(arg.name()|var_name|unquote) -}}
    {%- when Type::Record(_) %}
            let {{arg.type_()|type_name|var_name|unquote}} = try BreezSDKMapper.as{{arg.type_()|type_name}}(data: {{ arg.name()|var_name|unquote }})
    {%- else %}
    {%- endmatch %}
{%- endfor %}
{%- match func.return_type() -%}
{%- when Some with (return_type) %}
            let res = try {{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call swift::arg_list(func) -%})
    {%- match return_type %}
    {%- when Type::Optional(inner) %}
        {%- let unboxed = inner.as_ref() %}
            if res != nil {
                resolve({{ unboxed|rn_return_type(unboxed|type_name|var_name|unquote, true) }})
            } else {
                rejectErr(err: SdkError.Generic(message: "No result found"), reject: reject)
            }
    {%- else %}
            resolve({{ return_type|rn_return_type(return_type|type_name|var_name|unquote, false) }})
    {%- endmatch %}
{%- when None %}
            try {{ obj_interface }}{{ func.name()|fn_name|unquote }}({%- call swift::arg_list(func) -%})
            resolve(["status": "ok"])     
{%- endmatch %}
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }