# Gesture Recognizer

This crate provides abstract API to recognize and handle simple gestures. At now three type of gestures are supported:
* Move by **one finger**
* Scale and move by **two fingers**
* Move by **three fingers**

To handle this gestures you need to 
* implement `GestureEvents` trait
* has `GestureRecognizer` struct
* run `process` method of `GestureRecognizer` on each touch event, this `process` method will run methods of `GestureEvents` trait on object that you proceed to it

# miniquad

This crate originally created for [miniquad](https://github.com/not-fl3/miniquad). So, the crate has miniquad as optional dependency. If you choose this optional dependency, you will get code to transform `miniquad::TouchPhase` to `gesture_recognizer::TouchType` by `From` trait.

# TODO

## Gestures
* [ ] long-tap (user need to provide own time method (because of wasm, android etc.))
	* [ ] long-tap by N fingers
	* [ ] **recursive gestures** by long-tap (e.g. long-tap by one finger + then scale by two fingers or also long-tap by one finger + ...)
* [ ] discrete gestures
	* [ ] double-tap (user time method again)
		* [ ] N-tap by M-fingers
	* [ ] swipe ???
* [ ] screen edge flag ???

## Other
* [ ] add rotation to current two fingers gesture!
* [ ] add example on miniquad when all events are visualized and logged to screen and deploy it to wasm!
* [ ] stole ideas from https://developer.apple.com/documentation/uikit/uigesturerecognizer
* [ ] speed of gestures ???
* [ ] how to provide both powerful and easy interface ???
* [ ] think about point type, 

## Discarded
* recognize user images, eg smile, letters. Discarded because this need some machine learning and expensive computations