# Test Data Generation Workshop

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Coverage Status](https://coveralls.io/repos/github/dsietz/tdg-workshop/badge.svg?branch=master)](https://coveralls.io/github/dsietz/tdg-workshop?branch=master)
[![Docs.rs](https://docs.rs/tdg-workshop/badge.svg)](https://docs.rs/tdg-workshop)

Linux: [![Build Status](https://travis-ci.org/dsietz/tdg-workshop.svg?branch=master)](https://travis-ci.org/dsietz/tdg-workshop)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/5w1x4q7b8g29ijvi?svg=true)](https://ci.appveyor.com/project/dsietz/tdg-workshop/branch/master)

---

## Hands-On experience building a Test Data Generation Service

In this workshop we walk through the concepts, building blocks and the implementation of a light-weight Test Data Generation service that addresses this automated testing niche.

Continuous Integration has redefined our testing practices. Testing has become more focused, efficient, and re-positioned further upstream in the development life-cycle. Unfortunately, our testing systems haven't evolved in lock-step - specifically the provisioning of realist test data.

It remains common practice to extract, cleanse and load production data into our non- production environments. This is a lengthy process with serious security concerns, and still doesn't satisfy all our data content requirements. What if there is a better way of providing realist test data? What if it could be generated on-demand as part of the Continuous Integration process - without the heavy databases and traditional batch jobs?

>In this workshop you will learn how to 
>
>+ create RESTful services in Rust
>+ implement the [Test Data Generation SDK](https://crates.io/crates/test-data-generation)

---

__IMPORTANT:__ Participants need to ...

+ Bring their own development devices
+ Have installed Rust Toolchain prior to the workshop
+ Have internet connect during the workshop

---

## Developer Slice Setup
- [Rust Language](./docs/reference-rust.md)
- [Postman](./docs/reference-postman.md)

---

## Workshop Material
+ [Workshop Manual](https://davidsietz.gitbook.io/workspace/)
