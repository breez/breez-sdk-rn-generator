package com.breezsdk

import breez_sdk.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter
import java.io.File
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
{% import "macros.kt" as kt %}

class BreezSDKModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {
    private lateinit var executor: ExecutorService
    private var breezServices: BlockingBreezServices? = null

    companion object {
        const val TAG = "RNBreezSDK"
    }

    override fun initialize() {
        super.initialize()

        executor = Executors.newFixedThreadPool(3)
    }

    override fun getName(): String {
        return TAG
    }

    @Throws(SdkException::class)
    fun getBreezServices(): BlockingBreezServices {
        if (breezServices != null) {
            return breezServices!!
        }

        throw SdkException.Generic("BreezServices not initialized")
    }

    @ReactMethod
    fun addListener(eventName: String) {}

    @ReactMethod
    fun removeListeners(count: Int) {}

    {% let obj_interface = "" -%}
    {% for func in ci.function_definitions() %}
    {%- if func.name()|ignored_function == false -%}
    {% include "TopLevelFunctionTemplate.kt" %}
    {% endif -%}
    {%- endfor %}  
    @ReactMethod
    fun setLogStream(promise: Promise) {
        try {
            val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)

            setLogStream(BreezSDKLogStream(emitter))
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: Exception) {
            e.printStackTrace()
            promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
        }
    }

    @ReactMethod
    fun connect(config: ReadableMap, seed: ReadableArray, promise: Promise) {
        if (breezServices != null) {
            promise.reject("Generic", "BreezServices already initialized")
            return
        }

        try {
            val configTmp = asConfig(config) ?: run { throw SdkException.Generic("Missing mandatory field config of type Config") }
            val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)

            breezServices = connect(configTmp, asUByteList(seed), BreezSDKListener(emitter))
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: Exception) {
            e.printStackTrace()
            promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
        }
    }
    {%- include "Objects.kt" %}
}

