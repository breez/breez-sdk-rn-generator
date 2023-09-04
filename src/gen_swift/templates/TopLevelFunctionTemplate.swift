    
    @objc({%- call swift::extern_arg_list(func) -%})
    func {{ func.name()|fn_name|unquote }}(_ {% call swift::arg_list_decl(func) -%}resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) -> Void {
        do {
{%- for arg in func.arguments() -%}
    {%- match arg.type_() %}
    {%- when Type::Record(_) %}
            let {{arg.type_()|type_name|var_name|unquote}} = try as{{arg.type_()|type_name}}({{ arg.name()|var_name|unquote }})
    {%- else %}
    {%- endmatch %}
{%- endfor %}
            let res = try getBreezServices().{{ func.name()|fn_name|unquote }}({%- call swift::arg_list(func) -%})
{%- match func.return_type() -%}
{%- when Some with (return_type) %}
    {%- match return_type %}
    {%- when Type::Optional(inner) %}
        {%- let unboxed = inner.as_ref() %}
            if res != nil {
                resolve({% call swift::return_value(unboxed) %})
            } else {
                rejectErr(err: SdkError.Generic(message: "No result found"), reject: reject)
            }
    {%- else %}
            resolve({% call swift::return_value(return_type) %})
    {%- endmatch %}
{%- when None %}
            resolve(["status": "ok"])     
{%- endmatch %}
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }