# auth-tarball-from-git

Authenticate a tarball through a signed tag in a git repository (with
reproducible builds).

The signed git tag contains a hash of a commit object:

```
object a631953b1241368b5f6bc471f9d89948f985fcb3
type commit
tag openpgp/v1.9.0
tagger Justus Winter <justus@sequoia-pgp.org> 1653320477 +0200

openpgp: Release 1.9.0.
-----BEGIN PGP SIGNATURE-----

iQEzBAABCgAdFiEEJWpOVeSnLZetJGjniNx+MzhfeR0FAmKLqx0ACgkQiNx+Mzhf
eR1w7gf+MSS1Su+kclHSKpVCg03TTyVdg+zx95FTlQjBtGaSRMbOAoWvCX53hZm9
/w2YZJdHTGAR50hFj78xnQjPg8bSEYrQD6HaMc/TYlFkrQcPQULCV8aNiiTlKPUC
GC0L8OecqG1tILejLtWkJpoSAh+oAK0QKjgyy3bYZU+KzCinV2+TC8LaAvcBSngt
R/Xu9g8X6CYf88mfO+IAyGeaDD+JMyQFp6q1fgzlFx/lA31iIg49vf1b9yQo2fxA
y8hnYu+dztZNMRcEL7Cl5UgFnT4tDv/rDlNpM136KHyvrXaqYC0GhNEoAsXX975L
9o0OzzRPOAxJj9/4Wigvu/fhOWRXSA==
=8qk5
-----END PGP SIGNATURE-----
```

If we don't have a signed tarball but we do have a signed git tag we can use
this signature to prove authenticity of the tarball. To do this we verify the
signature, then attempt to generate an identical tarball from the commit
specified in the tag. This is possible because the output of `git archive` is
deterministic as long as the parameters are identical.

Using the source code from the tarball is preferable because it can be pinned
with modern cryptographic hash functions while git can only offer sha1.

Signature verification is done with [sequoia-pgp](https://sequoia-pgp.org/)
instead of gpg.

## ⚠️ Security Considerations ⚠️

Signed git tags only authenticate the **tag name**, not the **repository url**. A
`v0.1.0` tag can be replayed from one repository into another if they are both
signed by the key provided in `--keyring`.

The hash in the signed tag is a **SHA1** hash, which is known to be problematic
([2005], [2017], [2020]). Regardless of the quality of the pgp signature,
verifying a tarball with git can only provide sha1-tier cryptographic
properties.

[2005]: https://www.schneier.com/blog/archives/2005/02/sha1_broken.html
[2017]: https://shattered.io/
[2020]: https://www.ntu.edu.sg/news/detail/critical-flaw-demonstrated-in-common-digital-security-algorithm

## Usage

```sh
# Sequoia
$ wget https://keys.openpgp.org/vks/v1/by-fingerprint/CBCD8F030588653EEDD7E2659B7DD433F254904A
$ wget https://gitlab.com/sequoia-pgp/sequoia/-/archive/openpgp/v1.9.0/sequoia-openpgp-v1.9.0.tar.gz
$ auth-tarball-from-git --keyring CBCD8F030588653EEDD7E2659B7DD433F254904A --tag openpgp/v1.9.0 https://gitlab.com/sequoia-pgp/sequoia sequoia-openpgp-v1.9.0.tar.gz
[2022-05-27T19:28:50Z INFO  auth_tarball_from_git] Cloning repository from "https://gitlab.com/sequoia-pgp/sequoia"
[2022-05-27T19:28:54Z INFO  auth_tarball_from_git] Tag successfully verified
[2022-05-27T19:28:54Z INFO  auth_tarball_from_git] Reproducing archive...
[2022-05-27T19:28:55Z INFO  auth_tarball_from_git] Reading input that should be verified...
[2022-05-27T19:28:55Z INFO  auth_tarball_from_git] Comparing...
OK

# Monero
$ wget https://github.com/monero-project/monero/archive/refs/tags/v0.17.3.2.tar.gz
$ wget https://github.com/monero-project/monero/blob/master/utils/gpg_keys/luigi1111.asc
$ auth-tarball-from-git --keyring luigi1111.asc --tag v0.17.3.2 --prefix monero-0.17.3.2 https://github.com/monero-project/monero v0.17.3.2.tar.gz
[2022-05-27T19:30:03Z INFO  auth_tarball_from_git] Cloning repository from "https://github.com/monero-project/monero"
[2022-05-27T19:30:06Z INFO  auth_tarball_from_git] Tag successfully verified
[2022-05-27T19:30:06Z INFO  auth_tarball_from_git] Reproducing archive...
[2022-05-27T19:30:08Z INFO  auth_tarball_from_git] Reading input that should be verified...
[2022-05-27T19:30:08Z INFO  auth_tarball_from_git] Comparing...
OK
```

## Dependencies

Needs `sqv` from the sequoia-pgp project to be installed to verify pgp
signatures.

## Funding

This project was funded by myself with [github
sponsors](https://github.com/sponsors/kpcyrd).

## License

GPLv3+
