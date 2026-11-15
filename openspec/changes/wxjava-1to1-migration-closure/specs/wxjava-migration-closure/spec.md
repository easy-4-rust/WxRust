# WxJava Migration Closure Specification

## Requirements

### Requirement: Complete Source Inventory

The migration audit MUST enumerate every Java source file under the nine selected WxJava core Maven modules' `src/main/java` trees at the pinned Oracle commit. It MUST include package-root outliers such as `com.tencent.wework.Finance` and MUST classify package-info or unparsed files explicitly.

#### Scenario: Package-root outlier

- GIVEN a Java source file exists under a selected core module but outside its conventional package root
- WHEN the source inventory is generated
- THEN the file MUST appear exactly once with its actual package and FQN
- AND it MUST NOT be silently omitted.

### Requirement: Method Signature Inventory

The method inventory MUST identify each public/protected declared Java method using its JVM descriptor and MUST include return type, modifiers, parameter declaration, throws declaration, declaring class, overload identity, and Native status where applicable.

#### Scenario: Overloaded methods

- GIVEN one Java class declares multiple methods with the same name and different JVM descriptors
- WHEN the method inventory is generated
- THEN each overload MUST have a unique stable method_id and descriptor.

### Requirement: Explicit Migration Status

Every Java object and method MUST have exactly one primary migration status. Automatic name/path matching MUST NOT mark a method VERIFIED.

#### Scenario: Candidate mapping

- GIVEN a Java method name matches a Rust method candidate
- WHEN the compatibility matrix is generated
- THEN the row MUST remain CANDIDATE_MAPPED or NEEDS_REVIEW until behavior evidence is attached.

### Requirement: Unsupported Native SDK Surface

The Finance Native SDK surface MUST be explicitly represented. Unsupported native fetch/media/handle operations MUST be marked UNSUPPORTED, while pure Rust cryptographic substitutions MUST be marked ADAPTED until differential verification passes.

#### Scenario: Native media retrieval

- GIVEN a Finance media retrieval method is not implemented in Rust
- WHEN the matrix is generated
- THEN it MUST be marked UNSUPPORTED and MUST NOT contribute to verified parity.

### Requirement: Arithmetic Closure

Object and method summary reports MUST verify that status counts sum exactly to their denominators and that compatibility matrix rows equal method inventory rows.

#### Scenario: Inconsistent summary

- GIVEN a generated summary has status totals that do not equal the source denominator
- WHEN the audit runs
- THEN the audit MUST fail with a non-zero exit code and identify the inconsistent total.

### Requirement: Cancellation-safe Temporary Media File Ownership

The Rust MiniApp file-returning media download MUST own the temporary file and its per-download directory until the operation returns successfully. If the download fails or its future is cancelled before successful return, the implementation MUST close the file handle and remove both the partial file and its dedicated directory. Once a `PathBuf` is returned, cleanup ownership transfers to the caller, which MUST be able to read the complete file and explicitly remove the file and parent directory. This is an intentional Rust lifecycle adaptation of Java `FileUtils.createTmpFile/deleteOnExit`; it MUST NOT claim JVM-equivalent deletion timing.

#### Scenario: Cancel after a partial response chunk

- GIVEN the response has created a temporary file and at least one chunk has been written
- WHEN the download future is cancelled before returning
- THEN the file handle MUST be closed
- AND the partial file MUST be removed
- AND its dedicated temporary directory MUST be removed
- AND unrelated temporary files MUST remain untouched.

#### Scenario: Successful download transfers cleanup ownership

- GIVEN a valid Content-Disposition and a successful media response
- WHEN the download completes
- THEN the returned `PathBuf` MUST refer to a file containing the exact response bytes
- AND the file and directory MUST remain available until the caller removes them.

### Requirement: Backward-compatible Optional Rust File Download API

Adding the Rust file-returning media download method MUST NOT force existing downstream implementations of `WxMaMediaService` to implement a new method. The trait MUST provide an explicit default unsupported result, while the built-in MiniApp implementation MAY override it with the file-returning implementation.

#### Scenario: Existing downstream implementation

- GIVEN a downstream type implements the pre-existing upload and byte-returning media methods
- WHEN it is compiled against the updated `WxMaMediaService` trait
- THEN it MUST remain source-compatible without adding a file-returning method
- AND calling the default file-returning method MUST return an explicit unsupported error rather than fabricate a file.
