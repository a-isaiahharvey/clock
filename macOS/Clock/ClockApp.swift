//
//  ClockApp.swift
//  Clock
//
//  Created by Allister Harvey on 3/8/23.
//

import SwiftUI

@main
struct ClockApp: App {
  @AppStorage("ClockApp.tabSelection")
  private var tabSelection: Int = 3

  var body: some Scene {
    Window("Clock", id: "aiharveyClock") {
      ContentView(tabSelection: $tabSelection).frame(minWidth: 600, minHeight: 540).onAppear {
        NSWindow.allowsAutomaticWindowTabbing = false
      }
    }.commands {
      CommandGroup(before: .sidebar) {
        Button("Stopwatch") {
          tabSelection = 3
        }.keyboardShortcut(.init("3"), modifiers: .command)
        Button("Timer") {
          tabSelection = 4
        }.keyboardShortcut(.init("4"), modifiers: .command)
        Divider()
      }
    }
  }
}
