package com.breezsdk

import breez_sdk.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter
import java.io.File
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors

class BreezSDKModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {
    private lateinit var executor: ExecutorService
    private var breezServices: BlockingBreezServices? = null

    companion object {
        const val TAG = "RNBreezSDK"
        const val GENERIC_CODE = "Generic"
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
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(e.javaClass.simpleName, e.message, e)
        }
    }

    @ReactMethod
    fun connect(
        config: ReadableMap,
        seed: ReadableArray,
        logFilePath: String,
        promise: Promise,
    ) {
        if (breezServices != null) {
            promise.reject(TAG, "BreezServices already initialized")
            return
        }

        try {
            val configTmp = asConfig(config) ?: run { throw SdkException.Generic("Missing mandatory field config of type Config") }
            val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)
            val logFilePathTmp = logFilePath.takeUnless { it.isEmpty() }
            
            breezServices = connect(configTmp, asUByteList(seed), BreezSDKListener(emitter), BreezSDKNodeLogger(emitter), logFilePathTmp)
            promise.resolve(readableMapOf("status" to "ok"))
        } catch (e: SdkException) {
            e.printStackTrace()
            promise.reject(e.javaClass.simpleName, e.message, e)
        }
    }
    {%- include "Objects.kt" %}
}

{% import "macros.kt" as kt %}
