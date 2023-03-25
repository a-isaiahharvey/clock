//
//  RTimerView.swift
//  Clock
//
//  Created by Allister Harvey on 3/14/23.
//

import SwiftUI
import UserNotifications

struct CountdownView: View {
  @State var remainingTime: String = ""
  @Binding var timer: RTimer
  @Binding var isTimerRunning: Bool
  @Binding var isStarted: Bool

  var progress: Double {
    ((timer.remaining().asSecs() / timer.duration.asSecs()))
  }

  var body: some View {
    ZStack {
      Circle()
        .stroke(
          Color.pink.opacity(0.5),
          lineWidth: 7
        )
        .rotationEffect(.degrees(-90))
        .animation(.easeOut, value: progress)
      Circle()
        .trim(from: 0, to: progress)
        .stroke(
          Color.pink,
          lineWidth: 7
        )
        .rotationEffect(.degrees(-90))
        .animation(.easeOut, value: progress)
      VStack {
        Text(remainingTime).font(.system(size: 80)).fontWeight(.light).monospacedDigit()
      }
      .onAppear {
        let queue = DispatchQueue(label: "updateStringQueue")
        queue.async {
          while true {
            DispatchQueue.main.async {
              updateView()
            }
            if timer.isDone {
              timer.reset()
              scheduleNotification()
            }
            Thread.sleep(forTimeInterval: 0.03)
          }
        }
      }
    }
  }

  private func updateView() {
    self.remainingTime = Stopwatch.formatTimeString(duration: timer.remaining())
    self.isTimerRunning = self.timer.isRunning
    self.isStarted = !self.timer.hasNotStarted
  }

  private func scheduleNotification() {
    let content = UNMutableNotificationContent()
    content.title = "Clock"
    content.body = "Timer"
    content.interruptionLevel = .timeSensitive
    content.sound = .default

    let trigger = UNTimeIntervalNotificationTrigger(timeInterval: 0.1, repeats: false)
    let request = UNNotificationRequest(
      identifier: "timerDone", content: content, trigger: trigger)

    UNUserNotificationCenter.current().add(request) { error in
      if let error = error {
        print("Error scheduling notification: \(error.localizedDescription)")
      }
    }
  }
}

struct TimerPickerView: View {
  @Binding var timer: RTimer
  @Binding var timerStarted: Bool
  @AppStorage("TimerPickerView.hours") private var hours: Int = 0
  @AppStorage("TimerPickerView.minutes") private var minutes: Int = 5
  @AppStorage("TimerPickerView.seconds") private var seconds: Int = 0

  var body: some View {
    VStack {
      HStack {
        Picker(selection: $hours, label: Text("Hours")) {
          ForEach(0..<24) { hour in
            Text("\(hour)").tag("\(hour)")
          }
        }
        Picker(selection: $minutes, label: Text("Minutes")) {
          ForEach(0..<60) { hour in
            Text("\(hour)").tag("\(hour)")
          }
        }
        Picker(selection: $seconds, label: Text("Seconds")) {
          ForEach(0..<60) { hour in
            Text("\(hour)").tag("\(hour)")
          }
        }
      }
      Button(action: start) {
        Text("Start").padding()
      }
    }
  }
    
    private func start() {
        self.timer = RTimer(secs: calculateDateAsSeconds())
        self.timer.start()
        self.timerStarted = !self.timer.hasNotStarted
    }

  public func calculateDateAsSeconds() -> UInt64 {
    let hours = hours * 3_600
    let minutes = minutes * 60

    return UInt64(hours + minutes + seconds)
  }
}

struct RTimerView: View {
  @State private var rTimer: RTimer = RTimer(secs: 0)
  @State private var secs = 0
  @State private var isTimerRunning = false
  @State private var timerStarted = false
  @State private var timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

  var body: some View {
    VStack {
      if timerStarted {
        CountdownView(timer: $rTimer, isTimerRunning: $isTimerRunning, isStarted: $timerStarted)
          .frame(width: 350, height: 350)
        HStack(content: {
          Button(action: rTimer.reset) {
            Text("Done").padding()
          }.padding().disabled(rTimer.hasNotStarted)
          if isTimerRunning {
            Button(action: rTimer.stop) {
              Text("Stop").padding()
            }.padding()
          } else {
            Button(action: rTimer.start) {
                Text("Start").padding()
            }.padding()
          }
        })
      } else {
          TimerPickerView(timer: $rTimer, timerStarted: $timerStarted)
      }
    }.animation(.default, value: timerStarted)
  }
}

struct RTimerView_Previews: PreviewProvider {
  static var previews: some View {
    RTimerView()
  }
}
