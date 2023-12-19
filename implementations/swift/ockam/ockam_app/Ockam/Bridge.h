/* This file was autogenerated by cbindgen - DO NOT EDIT */

#ifndef OCKAM_APP_LIB_H
#define OCKAM_APP_LIB_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum C_Kind {
  Information = 0,
  Warning = 1,
  Error = 2,
} C_Kind;

typedef enum C_OrchestratorStatus {
  Disconnected = 0,
  Connecting,
  Connected,
  WaitingForToken,
  WaitingForEmailValidation,
  RetrievingSpace,
  RetrievingProject,
} C_OrchestratorStatus;

typedef struct C_Invitee {
  /**
   * Optional
   */
  const char *name;
  const char *email;
} C_Invitee;

typedef struct C_LocalService {
  const char *name;
  /**
   * Optional
   */
  const char *address;
  /**
   * Optional
   */
  uint16_t port;
  /**
   * Optional
   */
  const char *scheme;
  const struct C_Invitee *const *shared_with;
  uint8_t available;
} C_LocalService;

typedef struct C_Invitation {
  const char *id;
  const char *service_name;
  /**
   * Optional
   */
  const char *service_scheme;
  uint8_t accepting;
  uint8_t accepted;
  uint8_t ignoring;
} C_Invitation;

typedef struct C_Service {
  const char *id;
  const char *source_name;
  const char *address;
  uint16_t port;
  /**
   * Optional
   */
  const char *scheme;
  uint8_t available;
  uint8_t enabled;
} C_Service;

typedef struct C_ServiceGroup {
  const char *email;
  /**
   * Optional
   */
  const char *name;
  /**
   * Optional
   */
  const char *image_url;
  const struct C_Invitation *const *invitations;
  const struct C_Service *const *incoming_services;
} C_ServiceGroup;

typedef struct C_ApplicationState {
  uint8_t enrolled;
  uint8_t loaded;
  enum C_OrchestratorStatus orchestrator_status;
  /**
   * Optional
   */
  const char *enrollment_name;
  /**
   * Optional
   */
  const char *enrollment_email;
  /**
   * Optional
   */
  const char *enrollment_image;
  /**
   * Optional
   */
  const char *enrollment_github_user;
  const struct C_LocalService *const *local_services;
  const struct C_ServiceGroup *const *groups;
  const struct C_Invitee *const *sent_invitations;
} C_ApplicationState;

typedef struct C_Notification {
  enum C_Kind kind;
  const char *title;
  const char *message;
} C_Notification;

typedef struct C_RuntimeInformation {
  const char *version;
  const char *commit;
  /**
   * Optional, when overwritten
   */
  const char *home;
  /**
   * Optional, when overwritten
   */
  const char *controller_addr;
  /**
   * Optional, when overwritten
   */
  const char *controller_identity;
} C_RuntimeInformation;

/**
 * This functions initializes the application state.
 */
bool initialize_application(void (*application_state_callback)(struct C_ApplicationState state),
                            void (*notification_callback)(struct C_Notification notification));

/**
 * Accept the invitation with the provided id.
 */
void accept_invitation(const char *id);

/**
 * Ignore the invitation with the provided id.
 */
void ignore_invitation(const char *id);

/**
 * Initiate graceful shutdown of the application, exit process when complete.
 */
void shutdown_application(void);

/**
 * Share a local service with the provided emails.
 */
const char *share_local_service(const char *name, const char *emails);

/**
 * Enable an accepted service associated with the invite id.
 */
void enable_accepted_service(const char *invitation_id);

/**
 * Disable an accepted service associated with the invite id.
 */
void disable_accepted_service(const char *invitation_id);

/**
 * Removes a local service with the provided name.
 */
void delete_local_service(const char *name);

/**
 * Creates a local service with the provided name and address.
 */
const char *create_local_service(const char *name, const char *address);

/**
 * Synchronously resets the application state to a fresh installation.
 */
void reset_application_state(void);

/**
 * Starts user enrollment
 */
void enroll_user(void);

/**
 * This function retrieve the current version of the application state, for polling purposes.
 */
struct C_ApplicationState application_state_snapshot(void);

/**
 * This functions returns runtime information about the application.
 */
struct C_RuntimeInformation runtime_information(void);

/**
 * Free the runtime information memory
 */
void free_runtime_information(struct C_RuntimeInformation information);

/**
 * This function serves to create a mock application state for the UI.
 */
struct C_ApplicationState mock_application_state(void);

#endif /* OCKAM_APP_LIB_H */
