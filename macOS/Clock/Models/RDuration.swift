//
//  RDuration.swift
//  Clock
//
//  Created by Allister Harvey on 3/8/23.
//

import Foundation

public class RDuration {
    var ptr: UnsafeMutableRawPointer;
    
    init(value: UnsafeMutableRawPointer!) {
        self.ptr = value
    }
    
    func asSecs() -> UInt64 {
        rust_Duration_asSecs(ptr)
    }
    
    func asSecs() -> Double {
        rust_Duration_asSecsF64(ptr)
    }
    
    deinit {
        rust_Duration_free(ptr)
    }
}
