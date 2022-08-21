// MY VERSION
// Dont use, it's bad for your health!
// but it's faster (3% ish, sometimes 5%)
pub mod scary_speed {
    pub struct Writer<'a> {
        data: &'a mut Vec<u8>,
    }

    impl<'a> Writer<'a> {
        #[inline(always)]
        pub fn new(buffer: &'a mut Vec<u8>) -> Self {
            Writer { data: buffer }
        }
    }

    impl<'a> std::io::Write for Writer<'a> {
        #[inline(always)]
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let i = self.data.len();

            unsafe {
                std::ptr::copy_nonoverlapping(
                    buf.as_ptr(),
                    self.data.get_unchecked_mut(i),
                    buf.len(),
                );
                self.data.set_len(i + buf.len());
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}

// FROM TECHEMPOWER
pub mod tech_emp {
    use actix_web::web;
    /*
    Copyright (c) 2021, TechEmpower, Inc.
    All rights reserved.

    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:
        * Redistributions of source code must retain the above copyright
          notice, this list of conditions and the following disclaimer.
        * Redistributions in binary form must reproduce the above copyright
          notice, this list of conditions and the following disclaimer in the
          documentation and/or other materials provided with the distribution.
        * Neither the name TechEmpower, Inc. nor the names of its
          contributors may be used to endorse or promote products derived from
          this software without specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
    ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
    WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL TECHEMPOWER, INC. BE LIABLE FOR ANY DIRECT,
    INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
    BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
    DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
    OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
    NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
    EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
    */

    /* About 1.5% faster */
    pub struct Writer<'a, B>(pub &'a mut B);

    impl<'a, B: web::BufMut> std::io::Write for Writer<'a, B> {
        #[inline(always)]
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.put_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}
