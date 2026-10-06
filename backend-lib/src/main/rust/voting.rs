//! `zcash_voting` has no release on the NU7 librustzcash pre-releases this crate builds
//! against, so shielded voting is compiled out and every `VotingRustBackend` native throws.

use jni::{
    JNIEnv,
    objects::{JByteArray, JClass, JObject, JObjectArray, JString},
    sys::{JNI_FALSE, jboolean, jbyteArray, jint, jlong, jobject, jobjectArray, jstring},
};

use crate::utils::exception::unwrap_exc_or;

fn unavailable<T>(env: &mut JNIEnv, error_val: T) -> T {
    unwrap_exc_or(
        env,
        Ok(Err(anyhow::anyhow!(
            "Shielded voting is unavailable in this build"
        ))),
        error_val,
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_openVotingDbNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JString<'local>,
    _: JString<'local>,
    _: jint,
) -> jlong {
    unavailable(&mut env, 0)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_closeVotingDbNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
) {
    unavailable(&mut env, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_buildGovernancePcztNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
    _: jint,
    _: JObjectArray<'local>,
    _: JByteArray<'local>,
    _: JString<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_buildGovernancePcztFromSeedNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JString<'local>,
    _: jint,
    _: jint,
    _: JObjectArray<'local>,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
    _: JString<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_extractPcztSighashNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JByteArray<'local>,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_extractSpendAuthSigNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JByteArray<'local>,
    _: jint,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[cfg(feature = "android-test-fixtures")]
#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_extractPcztOutputRecipientFixtureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JByteArray<'local>,
    _: jint,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_storeWitnessesNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JObjectArray<'local>,
    _: JObjectArray<'local>,
) {
    unavailable(&mut env, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_precomputeDelegationPirNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JString<'local>,
    _: jint,
    _: jint,
    _: jint,
    _: jint,
    _: JObjectArray<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_buildAndProveDelegationNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JString<'local>,
    _: jint,
    _: jint,
    _: jint,
    _: jint,
    _: JObjectArray<'local>,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
    _: jint,
    _: JString<'local>,
    _: JObject<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getDelegationSubmissionNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JString<'local>,
    _: JString<'local>,
    _: JByteArray<'local>,
    _: JString<'local>,
    _: JByteArray<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getDelegationSubmissionWithKeystoneSigNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_storeKeystoneSignatureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[cfg(feature = "android-test-fixtures")]
#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_delegationProofResultFixtureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[cfg(feature = "android-test-fixtures")]
#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_storeDelegationProofFixtureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JByteArray<'local>,
) {
    unavailable(&mut env, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_computeBundleSetupNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JObjectArray<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_storeTreeStateNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: JByteArray<'local>,
) {
    unavailable(&mut env, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getWalletNotesNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JString<'local>,
    _: jlong,
    _: jint,
    _: JByteArray<'local>,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_generateNoteWitnessesNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JString<'local>,
    _: jint,
    _: JObjectArray<'local>,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_setupBundlesNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: JObjectArray<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_generateHotkeyNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JByteArray<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_storeDelegationTxHashNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JString<'local>,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getDelegationTxHashNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
) -> jstring {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_storeVoteTxHashNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
    _: JString<'local>,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_markVoteSubmittedNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getVoteTxHashNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
) -> jstring {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getCommitmentBundleNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_clearRecoveryStateNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_recordVcPositionNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
    _: jlong,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_recoverCommittedVoteNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_recordShareDelegationNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
    _: jint,
    _: JObjectArray<'local>,
    _: JByteArray<'local>,
    _: jlong,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getShareDelegationsNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getUnconfirmedDelegationsNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_markShareConfirmedNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
    _: jint,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_addSentServersNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
    _: jint,
    _: JObjectArray<'local>,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[cfg(feature = "android-test-fixtures")]
#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_storeVoteFixtureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jint,
    _: jint,
    _: jboolean,
) {
    unavailable(&mut env, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_initRoundNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jlong,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
    _: JByteArray<'local>,
    _: JString<'local>,
) {
    unavailable(&mut env, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getRoundStateNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_delegationPhasesNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_resetVotingSessionStateNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) {
    unavailable(&mut env, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_listRoundsNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getBundleCountNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) -> jint {
    unavailable(&mut env, 0)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_getVotesNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_clearRoundNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) {
    unavailable(&mut env, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_deleteSkippedBundlesNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
) -> jlong {
    unavailable(&mut env, 0)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_computeShareNullifierNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JByteArray<'local>,
    _: jint,
    _: JByteArray<'local>,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_scheduledShareSubmitAtNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: jlong,
    _: jlong,
    _: jboolean,
    _: JByteArray<'local>,
) -> jlong {
    unavailable(&mut env, 0)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_syncVoteTreeNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: JString<'local>,
) -> jlong {
    unavailable(&mut env, 0)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_resetTreeClientNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_storeVanPositionNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jlong,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_generateVanWitnessNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: jlong,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_warmProvingCachesNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) {
    unavailable(&mut env, ())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_extractOrchardFvkFromUfvkNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JString<'local>,
    _: jint,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_deriveHotkeyRawAddressNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JByteArray<'local>,
    _: jint,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[cfg(feature = "android-test-fixtures")]
#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_deriveHotkeyRawAddressForAccountFixtureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JByteArray<'local>,
    _: jint,
    _: jint,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_extractNcRootNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JByteArray<'local>,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_verifyWitnessNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JObject<'local>,
) -> jboolean {
    unavailable(&mut env, JNI_FALSE)
}

#[cfg(feature = "android-test-fixtures")]
#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_noteInfoArrayFixtureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[cfg(feature = "android-test-fixtures")]
#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_witnessDataArrayFixtureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[cfg(feature = "android-test-fixtures")]
#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_treeStateFixtureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[cfg(feature = "android-test-fixtures")]
#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_nonEmptyTreeStateFixtureNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) -> jbyteArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_buildSharePayloadsNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: JObject<'local>,
    _: jint,
    _: jint,
    _: jlong,
    _: jboolean,
) -> jobjectArray {
    unavailable(&mut env, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_cash_z_ecc_android_sdk_internal_jni_VotingRustBackend_buildVoteCommitmentNative<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    _: jlong,
    _: JString<'local>,
    _: jint,
    _: JByteArray<'local>,
    _: jint,
    _: jint,
    _: jint,
    _: JObject<'local>,
    _: jboolean,
    _: JObject<'local>,
) -> jobject {
    unavailable(&mut env, std::ptr::null_mut())
}
