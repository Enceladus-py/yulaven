package org.beratdalsuna.yulaven

import com.google.android.games.GameActivity
import android.os.Bundle

class MainActivity : GameActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
    }

    companion object {
        init {
            System.loadLibrary("yulaven")
        }
    }
}
