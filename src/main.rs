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

use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::future::Future;

use crossterm::event::KeyEvent;
use futures::future::BoxFuture;
use futures::StreamExt;

use crate::event::EventHandler;

mod event;

type ErrorResult<T = ()> = Result<T, &'static dyn StdError>;

#[tokio::main]
async fn main() -> ErrorResult {
    println!("Hello, world!");
    let event_handler = EventHandler::new([], [], [Box::pin(test)], [], [], []);
    event_handler.init().await?;
    Ok(())
}

fn test(key_code: &KeyEvent) -> BoxFuture<ErrorResult> {
    Box::pin(async move {
        println!("{key_code:?}");
        Ok(()) // Returning Ok(()) for successful execution
    })
}
