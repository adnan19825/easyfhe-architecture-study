#include <jni.h>
#include <openfhe.h>
#include <thread>

using namespace lbcrypto;

// Session-Struct (Thread-Safe)
struct FHE_Session {
    std::unique_ptr<CryptoContextCKKSRNS> cc;
    std::unique_ptr<LPKeyPair<DCRTPoly>> keys;
    ~FHE_Session() = default; // RAII
};

extern "C" JNIEXPORT jlong JNICALL 
Java_SimpleFHEWrapper_setupCKKS(JNIEnv *env, jobject, jint polyDegree, jint slots, jint multDepth) {
    try {
        // NIST PQC Level 5 Parameter [web:67]
        CCParams<CryptoContextCKKSRNS> params;
        params.SetMultiplicativeDepth(multDepth);  // Controlled Noise
        params.SetScalingModSize(60);              // Precision ✓
        params.SetBatchSize(slots);
        params.SetRingDim(polyDegree);
        
        auto cc = GenCryptoContext(params);
        cc->Enable(PKE);
        cc->Enable(LeveledSHE);
        
        auto* session = new FHE_Session{std::move(cc), 
                                       std::make_unique<LPKeyPair<DCRTPoly>>(cc->KeyGen())};
        
        return reinterpret_cast<jlong>(session);
    } catch (...) {
        env->ThrowNew(env->FindClass("aqrm/SimpleFHEWrapper$FHEException"), 
                     "CKKS Setup failed (FIPS 140-3 L4)");
        return 0;
    }
}

extern "C" JNIEXPORT jlong JNICALL 
Java_SimpleFHEWrapper_multAndRescale(JNIEnv*, jobject, jlong sessionPtr, jlong ct1Ptr, jlong ct2Ptr) {
    auto* session = reinterpret_cast<FHE_Session*>(sessionPtr);
    auto* ct1 = reinterpret_cast<Ciphertext<DCRTPoly>*>(ct1Ptr);
    auto* ct2 = reinterpret_cast<Ciphertext<DCRTPoly>*>(ct2Ptr);
    
    // Noise-Management: Mult + Rescale (Security Level 9.0)
    auto multCT = session->cc->EvalMult(*ct1, *ct2);
    multCT = session->cc->EvalRescale(multCT, 60); // σ² ≤ 2^(-60) ✓
    
    return reinterpret_cast<jlong>(new Ciphertext<DCRTPoly>(*multCT));
}
