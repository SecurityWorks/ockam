import SwiftUI

struct IncomingInvite: View {
    @State private var isHovered = false
    @State private var isOpen = false
    @ObservedObject var invite: Invitation
    @State var padding = 0.0

    var body: some View {
        let pending = !invite.accepting && !invite.ignoring
        VStack(alignment: .leading, spacing: 0) {
            HStack(spacing: 0) {
                Image(systemName: invite.accepting ? "envelope.open" : "envelope")
                    .padding(.trailing, StandardIconTextSpacing)

                VStack(alignment: .leading) {
                    Text(verbatim: invite.serviceName).lineLimit(1)
                    if invite.accepting {
                        Text(verbatim: "Accepting")
                            .font(.caption)
                            .foregroundStyle(OckamSecondaryTextColor)
                    } else if invite.ignoring {
                        Text(verbatim: "Declining")
                            .font(.caption)
                            .foregroundStyle(OckamSecondaryTextColor)
                    } else {
                        if let scheme = invite.serviceScheme {
                            Text(verbatim: scheme).font(.caption)
                                .foregroundStyle(OckamSecondaryTextColor)
                        }
                    }
                }
                Spacer()
                if pending {
                    Image(systemName: "chevron.right")
                        .rotationEffect(
                            isOpen ? Angle.degrees(90.0) : Angle.degrees(0), anchor: .center
                        )
                }
            }
            .padding(.leading, padding)
            .contentShape(Rectangle())
            .frame(height: VerticalSpacingUnit*4)
            .padding(.horizontal, HorizontalSpacingUnit)
            .onTapGesture {
                withAnimation {
                    if pending {
                        isOpen = !isOpen
                    }
                }
            }
            .onHover { hover in
                isHovered = hover
            }
            .background( isHovered ?
                AnyShapeStyle(HierarchicalShapeStyle.quaternary) :
                AnyShapeStyle(Color.clear)
            )
            .cornerRadius(4)
            .padding(.horizontal, WindowBorderSize)

            if isOpen {
                Divider()
                VStack(spacing: 0) {
                    if pending {
                        ClickableMenuEntry(
                            text: "Accept",
                            action: {
                                accept_invitation(invite.id)
                                isOpen = false
                            },
                            textPadding: padding + HorizontalSpacingUnit*2
                        )
                        ClickableMenuEntry(
                            text: "Decline",
                            action: {
                                ignore_invitation(invite.id)
                                isOpen = false
                            },
                            textPadding: padding + HorizontalSpacingUnit*2
                        )
                    }
                }
                .padding(.horizontal, WindowBorderSize)
                .background(HierarchicalShapeStyle.quinary)
                Divider()
            }
        }
    }
}



struct IncomingInvite_Previews: PreviewProvider {
    @State static var state = swift_demo_application_state()

    static var previews: some View {
        VStack(spacing: VerticalSpacingUnit) {
            IncomingInvite(invite: state.groups[0].invitations[0])
            IncomingInvite(invite: state.groups[0].invitations[1])
            IncomingInvite(invite: state.groups[1].invitations[1])
        }.frame(width: 320, height: 200)
    }
}
