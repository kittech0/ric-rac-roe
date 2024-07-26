/*
 * MIT License
 *
 * Copyright (c) 2024 Kittech
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */
use std::future::IntoFuture;
use std::marker::PhantomData;
use std::pin::Pin;

use crossterm::event::{Event, EventStream, KeyEvent, MouseEvent};
use futures::StreamExt;
use tokio::time;

use crate::ErrorResult;

type PinedBox<'a> = Pin<Box<dyn futures::Future<Output = ErrorResult> + 'a>>;
pub struct EventHandler<'a, FG, FL, K, M, P, R>
where
    FG: AsRef<[Pin<Box<dyn Fn() -> PinedBox<'a> + 'a>>]>,
    FL: AsRef<[Pin<Box<dyn Fn() -> PinedBox<'a> + 'a>>]>,
    K: AsRef<[Pin<Box<dyn Fn(&KeyEvent) -> PinedBox + 'a>>]>,
    M: AsRef<[Pin<Box<dyn Fn(&MouseEvent) -> PinedBox + 'a>>]>,
    P: AsRef<[Pin<Box<dyn Fn(&str) -> PinedBox + 'a>>]>,
    R: AsRef<[Pin<Box<dyn Fn(u16, u16) -> PinedBox<'a> + 'a>>]>,
{
    phantom_data: PhantomData<&'a ()>,
    pub focus_gained: FG,
    pub focus_lost: FL,
    pub key: K,
    pub mouse: M,
    pub paste: P,
    pub resize: R,
}

impl<'a, FG, FL, K, M, P, R> EventHandler<'a, FG, FL, K, M, P, R>
where
    FG: AsRef<[Pin<Box<dyn Fn() -> PinedBox<'a> + 'a>>]>,
    FL: AsRef<[Pin<Box<dyn Fn() -> PinedBox<'a> + 'a>>]>,
    K: AsRef<[Pin<Box<dyn Fn(&KeyEvent) -> PinedBox + 'a>>]>,
    M: AsRef<[Pin<Box<dyn Fn(&MouseEvent) -> PinedBox + 'a>>]>,
    P: AsRef<[Pin<Box<dyn Fn(&str) -> PinedBox + 'a>>]>,
    R: AsRef<[Pin<Box<dyn Fn(u16, u16) -> PinedBox<'a> + 'a>>]>,
{
    pub fn new(focus_gained: FG, focus_lost: FL, key: K, mouse: M, paste: P, resize: R) -> Self {
        Self {
            phantom_data: PhantomData::default(),
            focus_gained,
            focus_lost,
            key,
            mouse,
            paste,
            resize,
        }
    }

    pub async fn init(self) -> ErrorResult {
        let mut reader = EventStream::new();
        let mut interval = time::interval(time::Duration::from_millis(50));

        loop {
            interval.tick().await;
            match reader.next().await {
                Some(Ok(event)) => self.on_event(event),
                _ => continue,
            }
            .await?;
        }
    }

    async fn on_event(&self, event: Event) -> ErrorResult {
        match event {
            Event::FocusGained => {
                for run in self.focus_gained.as_ref() {
                    run().await?;
                }
            }

            Event::FocusLost => {
                for run in self.focus_lost.as_ref() {
                    run().await?;
                }
            }

            Event::Key(k) => {
                for run in self.key.as_ref() {
                    run(&k).await?;
                }
            }

            Event::Mouse(m) => {
                for run in self.mouse.as_ref() {
                    run(&m).await?;
                }
            }

            Event::Paste(p) => {
                for run in self.paste.as_ref() {
                    run(&p).await?;
                }
            }

            Event::Resize(x, y) => {
                for run in self.resize.as_ref() {
                    run(x, y).await?;
                }
            }
        };
        Ok(())
    }
}
