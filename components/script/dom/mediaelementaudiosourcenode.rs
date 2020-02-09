/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::audiocontext::AudioContext;
use crate::dom::audionode::AudioNode;
use crate::dom::bindings::codegen::Bindings::MediaElementAudioSourceNodeBinding;
use crate::dom::bindings::codegen::Bindings::MediaElementAudioSourceNodeBinding::MediaElementAudioSourceNodeMethods;
use crate::dom::bindings::codegen::Bindings::MediaElementAudioSourceNodeBinding::MediaElementAudioSourceOptions;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::reflector::reflect_dom_object;
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::htmlmediaelement::HTMLMediaElement;
use crate::dom::window::Window;
use dom_struct::dom_struct;
use servo_media::audio::media_element_source_node::MediaElementSourceNodeMessage;
use servo_media::audio::node::{AudioNodeInit, AudioNodeMessage};
use std::sync::mpsc;

#[dom_struct]
pub struct MediaElementAudioSourceNode {
    node: AudioNode,
    media_element: Dom<HTMLMediaElement>,
}

impl MediaElementAudioSourceNode {
    #[allow(unrooted_must_root)]
    fn new_inherited(
        context: &AudioContext,
        media_element: &HTMLMediaElement,
    ) -> Fallible<MediaElementAudioSourceNode> {
        let node = AudioNode::new_inherited(
            AudioNodeInit::MediaElementSourceNode,
            &*context.base(),
            Default::default(),
            0,
            1,
        )?;
        let (sender, receiver) = mpsc::channel();
        node.message(AudioNodeMessage::MediaElementSourceNode(
            MediaElementSourceNodeMessage::GetAudioRenderer(sender),
        ));
        let audio_renderer = receiver.recv().unwrap();
        media_element.set_audio_renderer(audio_renderer);
        let media_element = Dom::from_ref(media_element);
        Ok(MediaElementAudioSourceNode {
            node,
            media_element,
        })
    }

    #[allow(unrooted_must_root)]
    pub fn new(
        window: &Window,
        context: &AudioContext,
        media_element: &HTMLMediaElement,
    ) -> Fallible<DomRoot<MediaElementAudioSourceNode>> {
        let node = MediaElementAudioSourceNode::new_inherited(context, media_element)?;
        Ok(reflect_dom_object(
            Box::new(node),
            window,
            MediaElementAudioSourceNodeBinding::Wrap,
        ))
    }

    #[allow(non_snake_case)]
    pub fn Constructor(
        window: &Window,
        context: &AudioContext,
        options: &MediaElementAudioSourceOptions,
    ) -> Fallible<DomRoot<MediaElementAudioSourceNode>> {
        MediaElementAudioSourceNode::new(window, context, &*options.mediaElement)
    }
}

impl MediaElementAudioSourceNodeMethods for MediaElementAudioSourceNode {
    /// https://webaudio.github.io/web-audio-api/#dom-mediaelementaudiosourcenode-mediaelement
    fn MediaElement(&self) -> DomRoot<HTMLMediaElement> {
        DomRoot::from_ref(&*self.media_element)
    }
}
