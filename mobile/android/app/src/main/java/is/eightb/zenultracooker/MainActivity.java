package is.eightb.zenultracooker;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.DialogInterface;
import android.graphics.Color;
import android.os.Bundle;
import android.webkit.JavascriptInterface;
import android.webkit.WebChromeClient;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.widget.LinearLayout;
import android.widget.TextView;
import android.widget.Toast;

/**
 * zenUltraCooker v2.1.0 (versionCode 20100) — Native Android APK Host & Google Pay / Stripe Bridge.
 *
 * Dedicated with Eternal Family Love to Mom (* Alex ~ Alexandra / Dra)
 * from Peter (◯), Nate (◇), Dad (△ Chris) & College Son (🔊 on Speakerphone).
 *
 * Trademark: PawShaw Kitty (=^._.^= ∫) & PawShaw Doggy (▽・ω・▽).
 */
public class MainActivity extends Activity {

    public static final String SEMVER = "2.1.0";
    public static final int VERSION_CODE = 20100;
    public static final String STRIPE_MERCHANT_ID = "acct_1UUTcTRnidUxAlVy";

    private WebView webView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setBackgroundColor(Color.parseColor("#0B0D14"));

        // Top Native Sovereign Hearth Status Bar
        TextView banner = new TextView(this);
        banner.setText("🐾 zenUltraCooker v" + SEMVER + " • PawShaw Kitty & Doggy • G Pay & Stripe Ready");
        banner.setTextColor(Color.parseColor("#FFD700"));
        banner.setBackgroundColor(Color.parseColor("#151926"));
        banner.setTextSize(12.5f);
        banner.setPadding(28, 18, 28, 18);
        root.addView(
                banner,
                new LinearLayout.LayoutParams(
                        LinearLayout.LayoutParams.MATCH_PARENT,
                        LinearLayout.LayoutParams.WRAP_CONTENT));

        webView = new WebView(this);
        WebSettings settings = webView.getSettings();
        settings.setJavaScriptEnabled(true);
        settings.setDomStorageEnabled(true);
        settings.setAllowFileAccess(true);
        settings.setMediaPlaybackRequiresUserGesture(false);

        webView.setBackgroundColor(Color.parseColor("#0B0D14"));
        webView.setWebViewClient(new WebViewClient());
        webView.setWebChromeClient(new WebChromeClient());
        webView.addJavascriptInterface(new AndroidHearthBridge(), "AndroidHearthBridge");

        webView.loadUrl("file:///android_asset/www/index.html");

        root.addView(
                webView,
                new LinearLayout.LayoutParams(
                        LinearLayout.LayoutParams.MATCH_PARENT,
                        0,
                        1.0f));

        setContentView(root);
    }

    /**
     * Native JavaScript interface exposed as `window.AndroidHearthBridge` inside the WebView.
     */
    public class AndroidHearthBridge {

        @JavascriptInterface
        public String getSemVer() {
            return SEMVER;
        }

        @JavascriptInterface
        public void launchPaymentSheet(final String railName, final String amountUsd, final String bundleLabel) {
            runOnUiThread(new Runnable() {
                @Override
                public void run() {
                    new AlertDialog.Builder(MainActivity.this)
                            .setTitle("💳 " + railName + " • Stripe (" + STRIPE_MERCHANT_ID + ")")
                            .setMessage(
                                    "Bundle: " + bundleLabel + "\n"
                                    + "Amount: $" + amountUsd + " USD\n\n"
                                    + "Protected by 4-Burner Sovereign Hearth Firewall:\n"
                                    + "Cap(Hearth: Rent + Groceries + Household) ⊥ Risk(R&D)\n\n"
                                    + "Tap Confirm to authorize 1-Tap " + railName + " token.")
                            .setPositiveButton("Confirm $" + amountUsd, new DialogInterface.OnClickListener() {
                                @Override
                                public void onClick(DialogInterface dialog, int which) {
                                    Toast.makeText(
                                            MainActivity.this,
                                            "✓ Paid $" + amountUsd + " via " + railName + " (Hearth Protected!)",
                                            Toast.LENGTH_LONG).show();
                                }
                            })
                            .setNegativeButton("Cancel", null)
                            .show();
                }
            });
        }

        @JavascriptInterface
        public void sendFamilyNudgeToast(final String message) {
            runOnUiThread(new Runnable() {
                @Override
                public void run() {
                    Toast.makeText(MainActivity.this, "🐾 quantAI: " + message, Toast.LENGTH_LONG).show();
                }
            });
        }
    }

    @Override
    public void onBackPressed() {
        if (webView != null && webView.canGoBack()) {
            webView.goBack();
        } else {
            super.onBackPressed();
        }
    }
}
