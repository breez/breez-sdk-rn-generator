    
    @ReactMethod
    fun {{ func.name()|fn_name|unquote }}({%- call kt::arg_list_decl(func) -%}, promise: Promise) {
        executor.execute {
            try {
                val res = {{ func.name()|fn_name|unquote }}({%- call kt::arg_list(func) -%})
                {%- match func.return_type() -%}
                {%- when Some with (return_type) %}            
                promise.resolve({% call kt::return_value(return_type) %})
                {% when None %}
                promise.resolve(readableMapOf("status" to "ok"))
                {% endmatch %}
            } catch (e: SdkException) {
                promise.reject(e.javaClass.simpleName, e.message, e)
            }
        }
    }